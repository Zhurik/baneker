use scraper::{Html, Selector};
use std::io::Result;
use std::io::Write; // Для использования метода write_all
use std::process::{Command, Stdio};

mod constants;

#[tokio::main]
async fn main() -> Result<()> {
    let response = match reqwest::get(constants::BANEKS_RANDOM_URL).await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!(
                "Couldn't get response from {}: {}",
                constants::BANEKS_RANDOM_URL,
                e.to_string(),
            );
            std::process::exit(1);
        }
    };

    let raw_text = match response.text().await {
        Ok(raw) => raw,
        Err(e) => {
            eprintln!(
                "Couldn't parse response from {}: {}",
                constants::BANEKS_RANDOM_URL,
                e.to_string()
            );
            std::process::exit(1);
        }
    };

    let html = Html::parse_document(&raw_text);
    let selector = match Selector::parse("p") {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Couldn't find <p> in response: {}", e.to_string());
            std::process::exit(1);
        }
    };

    let raw_anek = match html.select(&selector).next() {
        Some(x) => x.inner_html(),
        None => {
            eprintln!("Missing anek in response");
            std::process::exit(1);
        }
    };

    let parts = raw_anek.split("<br>\n");
    let mut anek = String::new();
    for part in parts {
        anek = anek + part.trim();
    }

    let mut cowsay = match Command::new("cowsay")
        .arg("-r")
        .arg("-C")
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(x) => x,
        // TODO обработка разных вариантов
        Err(_) => {
            eprintln!("Couldn't start `cowsay`. Refer to docs to install.");
            std::process::exit(1);
        }
    };

    if let Some(stdin) = cowsay.stdin.as_mut() {
        stdin
            .write_all(anek.as_bytes())
            .expect("Failed to write to cowsay");
    }

    Ok(())
}
