use super::{arguments::_parse_args, io::_writeOutput, platforms::_platforms};
use ansi_term::Colour;
use futures::{stream::iter, StreamExt};
use reqwest::Client;
use tokio;

#[tokio::main]
pub async fn _takeover(hosts: Vec<String>, threads: usize) -> std::io::Result<()> {
    let client = &Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let args = &_parse_args();
    let fetches = iter(hosts.into_iter().map(|url| async move {
        match client.get(&url).send().await {
            Ok(resp) => match resp.text().await {
                Ok(text) => {
                    let platformName = _platforms(text);
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
                                _writeOutput(fileName.to_string(), outputData);
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
            },
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
    }))
    .buffer_unordered(threads)
    .collect::<Vec<()>>();
    fetches.await;
    /*
        In case you want to know how it works, here is a more simpler code explaining the overall workflow:
        let body = response.text().await?;
        if body.contains("<p><strong>There isn't a GitHub Pages site here.</strong></p>") {
            println!("GitHub Pages Sub-domain Takeover seems possible!");
        }
    */
    Ok(())
}
