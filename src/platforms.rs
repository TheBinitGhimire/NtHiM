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
		},
		{
			"platform": "Agile CRM",
			"content": "Sorry, this page is no longer available."
		},
		{
			"platform": "Anima",
			"content": "If this is your website and you've just created it, try refreshing in a minute"
		},
		{
			"platform": "Bitbucket",
			"content": "Repository not found"
		},
		{
			"platform": "Campaign Monitor",
			"content": "Trying to access your account?"
		},
		{
			"platform": "DigitalOcean",
			"content": "Domain uses DO name serves with no records in DO."
		},
		{
			"platform": "Fastly",
			"content": "Fastly error: unknown domain:"
		},
		{
			"platform": "Ghost",
			"content": "The thing you were looking for is no longer here, or never was"
		},
		{
			"platform": "HatenaBlog",
			"content": "404 Blog is not found"
		},
		{
			"platform": "Help Juice",
			"content": "We could not find what you're looking for."
		},
		{
			"platform": "HelpScout",
			"content": "No settings were found for this company:"
		},
		{
			"platform": "Heroku",
			"content": "No such app"
		},
		{
			"platform": "Intercom",
			"content": "Uh oh. That page doesn't exist."
		},
		{
			"platform": "JetBrains YouTrack",
			"content": "is not a registered InCloud YouTrack"
		},
		{
			"platform": "Kinsta",
			"content": "No Site For Domain"
		},
		{
			"platform": "LaunchRock",
			"content": "It looks like you may have taken a wrong turn somewhere. Don't worry...it happens to all of us."
		},
		{
			"platform": "Ngrok",
			"content": "Tunnel *.ngrok.io not found"
		},
		{
			"platform": "Pantheon",
			"content": "404 error unknown site!"
		},
		{
			"platform": "Pingdom",
			"content": "This public report page has not been activated by the user"
		},
		{
			"platform": "Readme.io",
			"content": "Project doesnt exist... yet!"
		},
		{
			"platform": "Shopify",
			"content": "Sorry, this shop is currently unavailable."
		},
		{
			"platform": "SmartJobBoard",
			"content": "This job board website is either expired or its domain name is invalid."
		},
		{
			"platform": "Strikingly",
			"content": "page not found"
		},
		{
			"platform": "Surge.sh",
			"content": "project not found"
		},
		{
			"platform": "Tumblr",
			"content": "Whatever you were looking for doesn't currently exist at this address"
		},
		{
			"platform": "Uberflip",
			"content": "Non-hub domain, The URL you've accessed does not provide a hub."
		},
		{
			"platform": "Unbounce",
			"content": "The requested URL was not found on this server."
		},
		{
			"platform": "UserVoice",
			"content": "This UserVoice subdomain is currently available!"
		},
		{
			"platform": "Webflow",
			"content": "The page you are looking for doesn't exist or has been moved."
		},
		{
			"platform": "Worksites",
			"content": "Hello! Sorry, but the website you&rsquo;re looking for doesn&rsquo;t exist."
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