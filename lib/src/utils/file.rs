use std::{
    fs::{self, File},
    io::BufReader,
};

use crate::models::ohlc::OHLC;

pub fn read_file(path: &str) -> BufReader<File> {
    let file = File::open(path).unwrap();
    BufReader::new(file)
}

pub fn write_file(path: &str, ohlc: &OHLC) {
    let mut data = fs::read_to_string(path).unwrap();
    let ohcl1 = serde_json::to_string(ohlc).unwrap();
    if data != "".to_string() {
        data.push_str(format!("\n {}", ohcl1).as_str());
    } else {
        data.push_str(format!("{}", ohcl1).as_str());
    }

    fs::write(path, data).expect("unable to write file");
}
