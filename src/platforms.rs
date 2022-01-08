use platform_dirs::AppDirs;

use serde::{de, Deserialize, Deserializer};

use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::Read;

#[derive(Deserialize)]
struct Response {
	platforms: Vec<Definition>,
}

#[derive(Deserialize)]
struct Definition {
	platform: String,
	#[serde(deserialize_with = "deserializeStringOrSequence")]
	content: Vec<String>,
}

fn deserializeStringOrSequence<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
	D: Deserializer<'de>,
{
	struct StringOrVec;

	impl<'de> de::Visitor<'de> for StringOrVec {
		type Value = Vec<String>;

		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			formatter.write_str("string or list of strings")
		}

		fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
		where
			E: de::Error,
		{
			Ok(vec![value.to_owned()])
		}

		fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
		where
			S: de::SeqAccess<'de>,
		{
			Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
		}
	}

	deserializer.deserialize_any(StringOrVec)
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
	cacheFile
		.read_to_string(&mut signatures)
		.expect("Unable to read the cache file!");
	return signatures;
}

pub fn _platforms(response: String) -> String {
	let _definitions = _get_signatures();
	let data: Response = serde_json::from_str(&_definitions).unwrap();
	let mut platformName: String = "None".to_string();
	for platform in data.platforms {
		for respText in platform.content {
			if response.contains(&respText) {
				platformName = platform.platform;
				break;
			}
		}
	}
	return platformName;
}
