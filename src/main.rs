mod debug;
mod fuzz;

use crate::fuzz::InputMutator;
use chrono::{DateTime, Utc};
use clap::Clap;
use ctrlc;
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdout, BufWriter, Error, Read, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Output, Stdio};
use std::{fs, io};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Weiwen Chen <17307110121@fudan.edu.cn>")]
struct Opts {
    fuzz_file: String,

    #[clap(short, long, default_value = "cfm_in/")]
    input: String,

    #[clap(short, long, default_value = "cfm_out/")]
    output: String,
}

fn get_inputs(input_path: &str) -> Result<Vec<Vec<u8>>, Error> {
    let i_files = fs::read_dir(input_path)?;
    let mut inputs = Vec::new();
    for i_path in i_files {
        let mut input = Vec::new();
        read_file_to_bytes(&i_path?.path(), &mut input)?;
        inputs.push(input);
    }
    Ok(inputs)
}

fn cc_handler() {
    print!("\n");
    println!("Ctrl-c is pressed");
    std::process::exit(0);
}

fn generate_mutators(inputs: &Vec<Vec<u8>>) -> Result<Vec<InputMutator>, Error> {
    Ok(inputs.into_iter().map(|s| InputMutator::new(&s)).collect())
}

fn run_fuzz_with_tmp_file(fuzz_file: &str, input: &[u8]) -> Result<Output, Error> {
    let mut tmp = File::create("./tmp_file")?;
    tmp.write_all(input)?;

    let child = Command::new(&fuzz_file)
        .stdin(Stdio::piped())
        .arg("./tmp_file")
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let mut child_stdin = (&child).stdin.as_ref().unwrap();
        let mut writer = BufWriter::new(&mut child_stdin);
        writer.write_all(input).unwrap();
        writer.flush().unwrap();
    }
    let output = child.wait_with_output();
    fs::remove_file("./tmp_file")?;
    output
}

fn exit_with_sigsegv(ecode: ExitStatus) -> bool {
    !(ecode.success() || !ecode.code().is_none())
}

fn write_output_files(output_file_path: &str, input: &[u8], exec_path: &str) -> Result<(), Error> {
    let now: DateTime<Utc> = Utc::now();
    let time_prefix = now.format("%Y-%m-%d-%H:%M:%S").to_string();

    let i = format!("{}/{}.input", output_file_path, time_prefix);
    let mut input_file = File::create(&i)?;
    input_file.write_all(input)?;

    let e = format!("{}/{}.exec", output_file_path, time_prefix);
    let mut exec_file = File::create(&e)?;
    exec_file.write_all(exec_path.as_bytes())?;
    Ok(())
}

fn read_file_to_bytes(path: &PathBuf, destination: &mut Vec<u8>) -> io::Result<()> {
    let mut f = File::open(path)?;
    let mut buffer = [0; 10];

    loop {
        let n = f.read(&mut buffer[..])?;
        if n == 0 {
            break;
        }
        destination.extend(&buffer[..n]);
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    ctrlc::set_handler(cc_handler).expect("Set ctrl-c handler failed.");

    let opts: Opts = Opts::parse();
    let inputs = get_inputs(&opts.input)?;
    let mut mutators: Vec<InputMutator> = generate_mutators(&inputs)?;

    let mut discovered_error_path: HashSet<String> = HashSet::new();

    // Loop all in round robin style.
    let mut unclassified_fault_count = 0;
    let mut run = 0;
    let mut unique_fault_count = 0;
    loop {
        for mutator in mutators.iter_mut() {
            mutator.mutate();
            let output = run_fuzz_with_tmp_file(&opts.fuzz_file, mutator.get_mutation())?;
            let ecode = output.status;
            print!(
                "\rtest {} times; fault count: {}; unique: {}; unclassified: {}",
                run,
                unique_fault_count + unclassified_fault_count,
                unique_fault_count,
                unclassified_fault_count
            );
            stdout().flush().unwrap();
            run += 1;
            if !exit_with_sigsegv(ecode) {
                continue;
            }
            let core_path = debug::get_first_core_name().expect("No core file in current dir");
            let exec_path = debug::get_exec_path(&opts.fuzz_file, &core_path);
            fs::remove_file(core_path).expect("No core file to remove!");
            if exec_path == "" || discovered_error_path.insert(exec_path.clone()) {
                write_output_files(&opts.output, mutator.get_mutation(), &exec_path)?;
                if exec_path == "" {
                    unclassified_fault_count += 1;
                }
                unique_fault_count = discovered_error_path.len();
            }
        }
    }
}
