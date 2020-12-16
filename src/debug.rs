use gdb;
use gdb::StreamRecord;
use regex::Regex;
use lazy_static::lazy_static;
use gdb::StreamRecord::Console;
use crate::debug::Error::ParseError;
use gdb::Record::Stream;

lazy_static! {
    static ref execute_location_re: Regex = Regex::new(r"^.+?(0x[a-z0-9]{16}) in (.+?) .+$").unwrap();
}

#[derive(Debug)]
struct ExecuteLocation {
    address: String,
    function: String,
}

#[derive(Debug)]
pub enum Error {
    ParseError,
}

impl ExecuteLocation {
    fn from_stream_record(s: &StreamRecord) -> Result<ExecuteLocation, Error>{
        let info_str = match s {
            Console(info_str) => info_str,
            _ => return Err(ParseError),
        };

        let caps = execute_location_re.captures(info_str.as_str()).unwrap();
        Ok(ExecuteLocation{ address: (&caps[1]).to_string(), function: (&caps[2]).to_string() })
    }
}

fn get_core_names(path: String) -> String {
    return String::from("./tests/hello/core.26772");
}

pub fn run_gdb() {
    // Run GDB with command
    let mut debugger = gdb::Debugger::start().unwrap();
    let response = debugger.send_cmd_raw("file ./tests/hello/flaw\n").unwrap();
    println!("{:?}", response);
    let response = debugger.send_cmd_raw("core ./tests/hello/core.26772\n").unwrap();
    println!("{:?}", response);
    let response = debugger.send_cmd_raw_full_record("bt\n").unwrap();
    println!("{:?}", &response);

    let location = match &response[1] {
        Stream(s) => ExecuteLocation::from_stream_record(s).unwrap(),
        _ => return
    };
    println!("location: {:?}", location);
    println!("Over");
}