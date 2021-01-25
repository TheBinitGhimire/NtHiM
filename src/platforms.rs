use ansi_term::{Colour, Style};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Response {
    platforms: Vec<Definition>,
}

#[derive(Deserialize)]
struct Definition {
    platform: String,
	content: String,
}

pub fn _platforms(url: String, response: String) -> serde_json::Result<()> {
	let _definitions = r#"
	{
		"platforms": [
		{
			"platform": "GitHub Pages",
			"content": "<p><strong>There isn't a GitHub Pages site here.</strong></p>"
		},
		{
			"platform": "WordPress.com",
			"content": "Do you want to register <em>"
		},
		{
			"platform": "Amazon S3",
			"content": "<Code>NoSuchBucket</Code>"
		}
		]
	}"#;
	let data: Response = serde_json::from_str(_definitions)?;
	for platform in data.platforms {
		if response.contains(&platform.content){
			println!("[{}]\t{} at {}!", Colour::Red.bold().paint(platform.platform), Colour::White.bold().paint("Possible Sub-domain Takeover"), url);
		}
	}
	
	Ok(())
}