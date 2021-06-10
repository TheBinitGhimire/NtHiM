#![allow(dead_code, non_snake_case, unused_must_use)]
mod arguments;
mod io;
mod platforms;
mod takeover;

use arguments::_parse_args;
use io::_fileRead;
use platforms::*;
use takeover::_takeover;

use platform_dirs::AppDirs;
use std::fs::remove_file;
use std::{path::Path, process::exit, string::String};

fn main() -> std::io::Result<()> {
	let app_dirs = AppDirs::new(Some("NtHiM"), true).unwrap();
	let cache_file_path = app_dirs.cache_dir.join("signatures.json");

	let args = _parse_args();
	if !cache_file_path.exists() || args.is_present("update") {
		println!("Updating signature cache, do wait till it gets cached!");
		let signatures = _get_signatures_from_repo().unwrap();
		_cache_signatures(signatures);
	}
	if args.is_present("output") {
		let fileName = args.value_of("output").unwrap();
		if Path::new(&fileName).exists() {
			remove_file(fileName).unwrap();
		}
	}
	let _threads: usize = args
		.value_of("threads")
		.unwrap_or("10")
		.parse::<usize>()
		.unwrap();
	if args.is_present("file") && args.is_present("target") {
		println!("Please provide either a single hostname or a file containing list of hostnames rather than both!");
		exit(1);
	} else if args.is_present("file") {
		let hostnames = args.value_of("file").unwrap_or("hostnames.txt");
		_fileRead(hostnames.to_string(), _threads);
	} else if args.is_present("target") {
		let _target = args.value_of("target").unwrap();
		let mut hosts = Vec::<String>::new();
		hosts.push(_target.to_string());
		_takeover(hosts, _threads);
	}

	Ok(())
}
