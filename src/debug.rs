use crate::debug::Error::ParseError;
use crate::debug::Error::NoCoreFile;
use gdb;
use gdb::Record::Stream;
use gdb::{StreamRecord, Record};
use gdb::StreamRecord::Console;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref execute_location_re: Regex =
        Regex::new(r"^.+?(0x[a-z0-9]{16}) in (.+?) .+$").unwrap();
    static ref core_name_re: Regex =
        Regex::new(r"^.+?core.[0-9]+?").unwrap();
    static ref function_name_re: Regex =
        Regex::new("\"(0x[0-9a-z]{16})  (.+?)(?:@.+?)?\\\\n\"").unwrap();
}

#[derive(Debug)]
struct ExecLocation {
    address: String,
    function: String,
}

#[derive(Debug)]
pub enum Error {
    ParseError,
    NoCoreFile,
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

// Only save named exec path.
fn exec_path_to_string(exec_locations: &Vec<ExecLocation>, func_names: &Vec<String>) -> String {
    let mut ans = "".to_string();
    for location in exec_locations {
        if location.function != "raise" && func_names.iter().any(|name| name.to_string() == location.function) {
            let s = &format!("{} {}\n", location.address, location.function);
            ans.push_str(s);
        }
    }
    ans
}

fn get_core_names(path: String) -> String {
    return String::from("./tests/hello/core.26772");
}

pub fn get_first_core_name() -> Option<String> {
    let paths = fs::read_dir("./").expect("Read current dir fail");
    for path in paths {
        let path_name = String::from(path.expect("Invalid path").path().to_str().unwrap());
        if core_name_re.is_match(&path_name) {
            return Some(path_name);
        }
    }
    None
}

pub fn get_exec_path(flaw_file_path: &str, dump_file_path: &str) -> String {
    let mut debugger = gdb::Debugger::start().unwrap();
    let file_cmd = &format!("file {}\n", flaw_file_path);
    debugger.send_cmd_raw(file_cmd).unwrap();

    let func_info = debugger.send_cmd_raw_full_record("info functions\n").unwrap();

    let func_names = parse_func_name(&func_info);

    // println!("{:?}", func_names);

    let core_cmd = &format!("core {}\n", dump_file_path);
    debugger.send_cmd_raw(core_cmd).unwrap();
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
    exec_path_to_string(&exec_locations, &func_names)
}

fn parse_func_name(func_info: &Vec<Record>) -> Vec<String> {
    let mut ans: Vec<String> = Vec::new();
    for r in func_info {
        match r {
            Stream(s) => {
                if let Console(c) = s {
                    if !function_name_re.is_match(c) {
                        continue
                    }
                    let caps = function_name_re.captures(c.as_str()).unwrap();
                    ans.push((&caps[2]).to_string());
                }
            }
            _ => continue
        }
    }
    ans
}