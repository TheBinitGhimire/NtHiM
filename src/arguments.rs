use clap::{App, AppSettings::ArgRequiredElseHelp, Arg, ArgMatches};

pub fn _parse_args() -> ArgMatches {
	App::new("NtHiM")
		.version("0.1.3")
		.author("Binit Ghimire <thebinitghimire@gmail.com>, Captain Nick Lucifer* <naryal2580@gmail.com>")
		.about("Now, the Host is Mine! - Super Fast Sub-domain Takeover Detection!")
		.setting(ArgRequiredElseHelp)
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
				.about("Update signature cache!")
				.short('u')
				.long("update")
				.takes_value(false),
		])
		.get_matches()
}
