use crate::debug::Error::ParseError;
use gdb;
use gdb::Record;
use gdb::Record::Stream;
use gdb::StreamRecord::Console;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref RE_EXECUTE_LOCATION: Regex =
        Regex::new(r"^.+?(0x[a-z0-9]{16}) in (.+?) .+$").unwrap();
    static ref RE_CORE_NAME: Regex = Regex::new(r"^.+?core.[0-9]+?").unwrap();
    static ref RE_FUNCTION_NAME: Regex =
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
}

impl ExecLocation {
    fn from_stream_record(s: &String) -> Result<ExecLocation, Error> {
        if !RE_EXECUTE_LOCATION.is_match(s.as_str()) {
            return Err(ParseError);
        }
        let caps = RE_EXECUTE_LOCATION.captures(s.as_str()).unwrap();
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
        if location.function != "raise"
            && func_names
                .iter()
                .any(|name| name.to_string() == location.function)
        {
            let s = &format!("{}\n", location.function);
            ans.push_str(s);
        }
    }
    ans
}

pub fn get_first_core_name() -> Option<String> {
    let paths = fs::read_dir("./").expect("Read current dir fail");
    for path in paths {
        let path_name = String::from(path.expect("Invalid path").path().to_str().unwrap());
        if RE_CORE_NAME.is_match(&path_name) {
            return Some(path_name);
        }
    }
    None
}

pub fn get_exec_path(flaw_file_path: &str, dump_file_path: &str) -> String {
    let mut debugger = gdb::Debugger::start().unwrap();
    let file_cmd = &format!("file {}\n", flaw_file_path);
    debugger.send_cmd_raw(file_cmd).unwrap();

    let func_info = debugger
        .send_cmd_raw_full_record("info functions\n")
        .unwrap();

    let func_names = parse_func_name(&func_info);
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
                    if !RE_FUNCTION_NAME.is_match(c) {
                        continue;
                    }
                    let caps = RE_FUNCTION_NAME.captures(c.as_str()).unwrap();
                    ans.push((&caps[2]).to_string());
                }
            }
            _ => continue,
        }
    }
    ans
}
