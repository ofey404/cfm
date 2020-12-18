use crate::debug::Error::ParseError;
use gdb;
use gdb::Record::Stream;
use gdb::StreamRecord;
use gdb::StreamRecord::Console;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref execute_location_re: Regex =
        Regex::new(r"^.+?(0x[a-z0-9]{16}) in (.+?) .+$").unwrap();
}

#[derive(Debug)]
struct ExecLocation {
    address: String,
    function: String,
}

#[derive(Debug)]
pub enum Error {
    ParseError,
}

impl ExecLocation {
    fn from_stream_record(s: &String) -> Result<ExecLocation, Error> {
        let caps = execute_location_re.captures(s.as_str()).unwrap();
        Ok(ExecLocation {
            address: (&caps[1]).to_string(),
            function: (&caps[2]).to_string(),
        })
    }
}

fn exec_path_to_string(exec_locations: &Vec<ExecLocation>) -> String {
    let mut ans = "".to_string();
    for location in exec_locations {
        let s = &format!("{} {}\n", location.address, location.function);
        ans.push_str(s);
    }
    ans
}

fn get_core_names(path: String) -> String {
    return String::from("./tests/hello/core.26772");
}

pub fn get_first_core_name() -> String {
    return String::from("./tests/hello/core.26772");
}

pub fn get_exec_path(flaw_file_path: &str, dump_file_path: &str) -> String {
    let mut debugger = gdb::Debugger::start().unwrap();
    let file_cmd = &format!("file {}\n", flaw_file_path);
    let response = debugger.send_cmd_raw(file_cmd).unwrap();
    let core_cmd = &format!("core {}\n", dump_file_path);
    let response = debugger.send_cmd_raw(core_cmd).unwrap();
    let response = debugger.send_cmd_raw_full_record("bt\n").unwrap();

    let mut exec_locations = Vec::new();

    for record in &response {
        match &record {
            Stream(s) => match s {
                Console(c) => exec_locations.push(ExecLocation::from_stream_record(c).unwrap()),
                _ => continue,
            },
            _ => continue,
        };
    }
    exec_path_to_string(&exec_locations)
}

pub fn run_gdb() {
    // // Run GDB with command
    // let mut debugger = gdb::Debugger::start().unwrap();
    // let response = debugger.send_cmd_raw("file ./tests/hello/flaw\n").unwrap();
    // println!("{:?}", response);
    // let response = debugger
    //     .send_cmd_raw("core ./tests/hello/core.26772\n")
    //     .unwrap();
    // println!("{:?}", response);
    // let response = debugger.send_cmd_raw_full_record("bt\n").unwrap();
    // println!("{:?}", &response);
    //
    // let mut exec_locations = Vec::new();
    //
    // for record in &response {
    //     let location = match &record {
    //         Stream(s) => match s {
    //             Console(c) => exec_locations.push(ExecLocation::from_stream_record(c).unwrap()),
    //             _ => continue,
    //         }
    //         _ => continue,
    //     };
    // }
    // println!("{}", exec_path_to_string(&exec_locations));
    // println!("Over");

    println!(
        "{}",
        get_exec_path("./tests/hello/flaw", "tests/hello/core.26772")
    );
}
