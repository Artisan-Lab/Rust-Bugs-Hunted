extern crate json;

fn _read_data()-> Vec<u8> {
    use std::path::PathBuf;
    let crash_file_name = "data/replay_json53-1";
    let crash_path = PathBuf::from(crash_file_name);
    use std::fs;
    let data =  fs::read(crash_path).unwrap();
    data
}

fn main() {
    let data = _read_data();
    let _param0 = std::str::from_utf8(&data).unwrap();
    let _ = json::parse(_param0);
}