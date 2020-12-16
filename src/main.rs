mod debug;

use clap::Clap;
use std::process::{Command, Stdio};
use std::fs::File;
use std::io::{Read, Write, Error, BufWriter};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Weiwen Chen <17307110121@fudan.edu.cn>")]
struct Opts {
    fuzz_file: String,

    #[clap(short, long, default_value = "cfm_in/")]
    input: String,

    #[clap(short, long, default_value = "cfm_out/")]
    output: String,
}

fn main() -> Result<(), Error> {
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