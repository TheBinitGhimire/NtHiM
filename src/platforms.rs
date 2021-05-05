use serde::{Deserialize};

#[derive(Deserialize)]
struct Response {
	platforms: Vec<Definition>,
}

#[derive(Deserialize)]
struct Definition {
	platform: String,
	content: String,
}

pub fn _platforms(response: String) -> String {
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
			"content": "<title>No such app</title>"
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
			"content": "You have successfully pointed this domain to Kinsta servers but you haven't added it to any of your sites."
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
			"content": "<h2>Project doesnt exist... yet!</h2>"
		},
		{
			"platform": "Shopify",
			"content": "<h1 class=\"tc\">Sorry, this shop is currently unavailable.</h1>"
		},
		{
			"platform": "SmartJobBoard",
			"content": "This job board website is either expired or its domain name is invalid."
		},
		{
			"platform": "Strikingly",
			"content": "<title>Page not found - Strikingly</title>"
		},
		{
			"platform": "Surge.sh",
			"content": "<h1>project not found</h1>"
		},
		{
			"platform": "Tumblr",
			"content": "Whatever you were looking for doesn't currently exist at this address. Unless you were looking for this error page, in which case: Congrats! You totally found it."
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
			"content": "<p class=\"description\">The page you are looking for doesn't exist or has been moved.</p>"
		},
		{
			"platform": "Worksites",
			"content": "<p>Hello! Sorry, but the website you&rsquo;re looking for doesn&rsquo;t exist.</p>"
		},
		{
			"platform": "WordPress.com",
			"content": "<title>Error: Domain mapping upgrade for this domain not found</title>"
	
		}	
		]
	}"#;
	let data: Response = serde_json::from_str(_definitions).unwrap();
	let mut platformName: String = "None".to_string();
	for platform in data.platforms {
		if response.contains(&platform.content){
			platformName = platform.platform;
			break
		}
	}
	return platformName
}
