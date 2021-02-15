#![allow(non_snake_case, unused_must_use)]
mod platforms;

use std::{process::exit, string::String};
use clap::{Arg, App};
use ansi_term::{Colour};

use futures::{StreamExt};

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
	let args = App::new("NtHiM")
			.version("0.0.4")
			.author("Binit Ghimire <binit@WHOISbinit.me>")
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
				]).get_matches();
				
	let _threads: usize = args.value_of("threads").unwrap_or("10").parse::<usize>().unwrap();
	if args.is_present("file") && args.is_present("target") {
		println!("Please provide either a single hostname or a file containing list of hostnames rather than both!");
		exit(1);
	} else if args.is_present("file"){
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

fn fileRead(filepath: String, threads: usize){
	let file = File::open(filepath).unwrap();
	let reader = BufReader::new(file);
	let mut hosts = Vec::<String>::new();
	for line in reader.lines(){
		hosts.push(line.unwrap());
	}
	takeover(hosts, threads);
}

#[tokio::main]
async fn takeover(hosts: Vec<String>, threads: usize) -> std::io::Result<()> {
	let fetches = futures::stream::iter(
		hosts.into_iter().map(|url| {
			async move {
				match reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap().get(&url).send().await {
					Ok(resp) => {
						match resp.text().await {
							Ok(text) => {
								platforms::_platforms(url, text);
							}
							Err(_) => println!("[{}]\tAn error occured for [{}].", Colour::Green.bold().paint("ERROR"), Colour::White.bold().paint(url)),
						}
					}
					Err(_) => println!("[{}]\tThe host {} appears to be down!", Colour::Green.bold().paint("ERROR"), Colour::White.bold().paint(url)),
				}
			}
	})
	).buffer_unordered(threads).collect::<Vec<()>>();
	fetches.await;
	/*	In case you want to know how it works, here is a more simple code:
		let body = res.text().await?;
		if body.contains("<p><strong>There isn't a GitHub Pages site here.</strong></p>") {
			println!("GitHub Pages Sub-domain Takeover seems possible!");
		} */
    Ok(())
}
