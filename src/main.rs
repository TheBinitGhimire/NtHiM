#![allow(non_snake_case, unused_must_use)]
mod platforms;

use ansi_term::Colour;
use clap::{App, Arg, ArgMatches};
use std::{path::Path, process::exit, string::String};

use futures::StreamExt;

use std::fs::{remove_file, File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, BufWriter};

use platform_dirs::AppDirs;

fn parse_args() -> ArgMatches {
	App::new("NtHiM")
		.version("0.1.1")
		.author("Binit Ghimire <binit@WHOISbinit.me>, Captain Nick Lucifer* <naryal2580@gmail.com>")
		.about("Now, the Host is Mine! - Super Fast Sub-domain Takeover Detection!")
		.args(&[
			Arg::new("file")
				.about("List of Hostnames separated with new line (\\n)!")
				.short('f')
				.long("file")
				.takes_value(true),
			Arg::new("target")
				.about("Hostname with the protocol defined!")
				.short('t')
				.long("target")
				.takes_value(true),
			Arg::new("threads")
				.about("Number of Concurrent Threads!")
				.short('c')
				.long("threads")
				.takes_value(true),
			Arg::new("verbose")
				.about("Enable Verbose Mode!")
				.short('v')
				.long("verbose")
				.takes_value(false),
			Arg::new("output")
				.about("Write output to file!")
				.short('o')
				.long("output")
				.takes_value(true),
			Arg::new("update")
				.about("Update signature cache.")
				.short('u')
				.long("update")
				.takes_value(false),
		])
		.get_matches()
}

fn main() -> std::io::Result<()> {
	let app_dirs = AppDirs::new(Some("NtHiM"), true).unwrap();
	let cache_file_path = app_dirs.cache_dir.join("signatures.json");
	let args = parse_args();
	if !cache_file_path.exists() || args.is_present("update") {
		println!("Updating signature cache, do wait till it get cached.");
		let signatures = platforms::_get_signatures_from_repo().unwrap();
		platforms::_cache_signatures(signatures);
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
		fileRead(hostnames.to_string(), _threads);
	} else if args.is_present("target") {
		let _target = args.value_of("target").unwrap();
		let mut hosts = Vec::<String>::new();
		hosts.push(_target.to_string());
		takeover(hosts, _threads);
	}

	Ok(())
}

fn fileRead(filepath: String, threads: usize) {
	let file = File::open(filepath).unwrap();
	let reader = BufReader::new(file);
	let mut hosts = Vec::<String>::new();
	for line in reader.lines() {
		hosts.push(line.unwrap());
	}
	takeover(hosts, threads);
}

fn writeOutput(fileName: String, outputData: String) {
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

#[tokio::main]
async fn takeover(hosts: Vec<String>, threads: usize) -> std::io::Result<()> {
	let fetches = futures::stream::iter(hosts.into_iter().map(|url| async move {
		match reqwest::Client::builder()
			.danger_accept_invalid_certs(true)
			.build()
			.unwrap()
			.get(&url)
			.send()
			.await
		{
			Ok(resp) => {
				let args = parse_args();
				match resp.text().await {
					Ok(text) => {
						let platformName = platforms::_platforms(text);
						match platformName == "None" {
							true => {
								if args.is_present("verbose") {
									println!(
										"[{}] {}!",
										Colour::Blue.bold().paint("Not Vulnerable"),
										url
									);
								}
							}
							_ => {
								println!(
									"[{}]\t{} at {}!",
									Colour::Red.bold().paint(&platformName),
									Colour::White.bold().paint("Possible Sub-domain Takeover"),
									url
								);
								if args.is_present("output") {
									let outputData = format!("[{}] {}\n", platformName, url);
									let fileName = args.value_of("output").unwrap();
									writeOutput(fileName.to_string(), outputData);
								}
							}
						}
					}
					Err(_) => {
						if args.is_present("verbose") {
							println!(
								"[{}]\tAn error occured for [{}].",
								Colour::Green.bold().paint("ERROR"),
								Colour::White.bold().paint(url)
							)
						}
					}
				}
			}
			Err(_) => (),
		}
	}))
	.buffer_unordered(threads)
	.collect::<Vec<()>>();
	fetches.await;
	/*	In case you want to know how it works, here is a more simpler code without any chik-boom:
	let body = res.text().await?;
	if body.contains("<p><strong>There isn't a GitHub Pages site here.</strong></p>") {
		println!("GitHub Pages Sub-domain Takeover seems possible!");
	} */
	Ok(())
}
