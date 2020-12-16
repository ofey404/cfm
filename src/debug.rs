use gdb;
use gdb::StreamRecord;

#[derive(Debug)]
struct ExecuteLocation {
    address: u64,
    function: String,
}

// #[derive(Debug)]
// pub enum Error {
// }
//
// impl ExecuteLocation {
//     fn from_stream_record(s: StreamRecord) -> Result<ExecuteLocation, Error>{
//
//     }
// }
//
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
    println!("{:?}", response);
}