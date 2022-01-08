use platform_dirs::AppDirs;

use serde::Deserialize;

use std::error::Error;
use std::fs::{self, File};
use std::io::Read;

#[derive(Deserialize)]
struct Response {
	platforms: Vec<Definition>,
}

#[derive(Deserialize)]
struct Definition {
	platform: String,
	content: String,
}

pub fn _get_signatures_from_repo() -> Result<String, Box<dyn Error>> {
	let url = "https://git.io/signatures_json";
	let resp = reqwest::blocking::get(url)?.text()?;
	Ok(resp)
}

pub fn _cache_signatures(signatures: String) {
	let app_dirs = AppDirs::new(Some("NtHiM"), true).unwrap();
	let cache_file_path = app_dirs.cache_dir.join("signatures.json");
	fs::create_dir_all(&app_dirs.cache_dir).unwrap();
	fs::write(cache_file_path, signatures).expect("Unable to cache signatures!");
}

pub fn _get_signatures() -> String {
	let app_dirs = AppDirs::new(Some("NtHiM"), true).unwrap();
	let cache_file_path = app_dirs.cache_dir.join("signatures.json");
	let mut signatures = String::new();
	let mut cacheFile = File::open(cache_file_path).expect("Unable to open cache file!");
	cacheFile.read_to_string(&mut signatures)
		.expect("Unable to read the cache file!");
	return signatures;
}

pub fn _platforms(response: String) -> String {
	let _definitions = _get_signatures();
	let data: Response = serde_json::from_str(&_definitions).unwrap();
	let mut platformName: String = "None".to_string();
	for platform in data.platforms {
		if response.contains(&platform.content) {
			platformName = platform.platform;
			break;
		}
	}
	return platformName;
}
