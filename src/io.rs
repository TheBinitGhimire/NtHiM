use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};

pub fn _fileRead(filepath: String) -> Vec<String> {
    let file = File::open(filepath).expect("Unable to open file!");
    let reader = BufReader::new(file);
    let mut hosts = Vec::<String>::new();
    for line in reader.lines() {
        match line {
            Ok(line) => hosts.push(line),
            Err(_) => ()
        }
    }
    return hosts;
}

pub fn _writeOutput(fileName: String, outputData: String) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(fileName)
        .expect("Unable to open file!");
    let mut file = BufWriter::new(file);
    file.write_all(outputData.as_bytes())
        .expect("Unable to write data to file!");
}
