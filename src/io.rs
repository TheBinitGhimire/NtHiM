use super::takeover::_takeover;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};

pub fn _fileRead(filepath: String, threads: usize) {
	let file = File::open(filepath).unwrap();
	let reader = BufReader::new(file);
	let mut hosts = Vec::<String>::new();
	for line in reader.lines() {
		hosts.push(line.unwrap());
	}
	_takeover(hosts, threads);
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
