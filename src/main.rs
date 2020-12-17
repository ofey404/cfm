mod debug;
mod fuzz;

use crate::fuzz::InputMutator;
use clap::Clap;
use ctrlc;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, Read, Write};
use std::process::{Command, Stdio, Output};
use std::thread::sleep;
use std::{fs, time};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Weiwen Chen <17307110121@fudan.edu.cn>")]
struct Opts {
    fuzz_file: String,

    #[clap(short, long, default_value = "cfm_in/")]
    input: String,

    #[clap(short, long, default_value = "cfm_out/")]
    output: String,
}

fn get_inputs(input_path: &str) -> Result<Vec<String>, Error> {
    let i_files = fs::read_dir(input_path)?;
    let mut inputs = Vec::new();
    for i_path in i_files {
        let mut input = String::new();
        let mut i_file = File::open(i_path?.path())?;
        i_file.read_to_string(&mut input)?;
        inputs.push(input);
    }
    Ok(inputs)
}

fn cc_handler() {
    println!("Ctrl-c is pressed");
    std::process::exit(0);
}

fn generate_mutators(inputs: &Vec<String>) -> Result<Vec<InputMutator>, Error> {
    Ok(inputs
        .into_iter()
        .map(|s| InputMutator::new(&s).expect("InputMutator initialization failed!"))
        .collect())
}

fn run_fuzz(fuzz_file: &str, input: &str) -> Result<Output, Error> {
    let child = Command::new(&fuzz_file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let mut child_stdin = (&child).stdin.as_ref().unwrap();
        let mut writer = BufWriter::new(&mut child_stdin);
        writer.write_all(input.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
    child.wait_with_output()
}

fn main() -> Result<(), Error> {
    ctrlc::set_handler(cc_handler).expect("Set ctrl-c handler failed.");

    let opts: Opts = Opts::parse();
    let mut inputs = get_inputs(&opts.input)?;
    let mut mutators: Vec<InputMutator> = generate_mutators(&inputs)?;

    // Loop all in round robin style.
    loop {
        for mutator in mutators.iter_mut() {
            mutator.mutate();
            let output = run_fuzz(&opts.fuzz_file, mutator.get_mutation())?;
            // TODO: Analyse.
            // TODO: Update output.
        }
        sleep(time::Duration::from_secs(2));
    }

    Ok(())
}

// TODO: Sketch, Waiting to be removed
fn __main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    println!("Input: {}", opts.input);
    println!("Output: {}", opts.output);
    println!("Fuzzfile: {}", opts.fuzz_file);

    // Read a input file
    let mut input_file = File::open(&opts.input)?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    println!("File content:{}", &input);

    // Write a output file
    {
        let mut output_file = File::create(opts.output)?;
        output_file.write_all(b"This is output")?;
    }

    // Fire a subroutine, pass argument to it.
    let child = Command::new(opts.fuzz_file)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let mut child_stdin = (&child).stdin.as_ref().unwrap();
        let mut writer = BufWriter::new(&mut child_stdin);
        writer.write_all(input.as_bytes()).unwrap();
        writer.flush().unwrap();
    }

    // Collect its output and its return value.
    let output = child.wait_with_output()?;

    println!("{}", String::from_utf8(output.stdout).unwrap());

    let ecode = output.status;
    // Check its return value
    if ecode.success() {
        println!("flaw run successful");
    } else {
        match ecode.code() {
            Some(code) => println!("flaw failed with code {}", code),
            None => println!("process terminated by signal"),
        }
    }

    debug::run_gdb();

    Ok(())
}
