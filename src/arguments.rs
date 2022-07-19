use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, ArgMatches};

pub fn _parse_args() -> ArgMatches {
    App::new("NtHiM")
		.version("0.2.4")
		.author("Late Binit Ghimire <thebinitghimire@gmail.com>, Captain Nick Lucifer* <naryal2580@gmail.com>")
		.about("Now, the Host is Mine! - Super Fast Sub-domain Takeover Detection!")
		.setting(ArgRequiredElseHelp)
		.args(&[
			Arg::new("file")
				.help("List of Hostnames separated with new line (\\n)!")
				.short('f')
				.long("file")
				.takes_value(true),
			Arg::new("target")
				.help("Hostname with the protocol defined!")
				.short('t')
				.long("target")
				.takes_value(true),
			Arg::new("platforms")
				.help("Path to custom signatures.json/platforms.json file.")
				.short('p')
				.long("platforms")
				.takes_value(true),
			Arg::new("threads")
				.help("Number of Concurrent Threads! (default: 10)")
				.short('c')
				.long("threads")
				.takes_value(true),
			Arg::new("timeout")
				.help("Timeout for connections (in seconds)! (default: 5)")
				.short('s')
				.long("timeout")
				.takes_value(true),
			Arg::new("verbose")
				.help("Enable Verbose Mode!")
				.short('v')
				.long("verbose")
				.takes_value(false),
			Arg::new("output")
				.help("Write output to file!")
				.short('o')
				.long("output")
				.takes_value(true),
			Arg::new("update")
				.help("Update signature cache!")
				.short('u')
				.long("update")
				.takes_value(false),
		])
		.get_matches()
}
