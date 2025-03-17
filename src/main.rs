use scraper::{Html, Selector};
use std::process::{Command, Stdio};
use std::io::Write; // Для использования метода write_all

mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(constants::BANEKS_RANDOM_URL).await?.text().await?;
    let html = Html::parse_document(&resp);
    let selector = Selector::parse("p").unwrap();

    let val = html.select(&selector).next().unwrap().inner_html();

    let parts = val.split("<br>\n");
    let mut anek = String::new();
    for part in parts {
        anek = anek + part.trim();
    }

    let mut cowsay = Command::new("cowsay")
        .arg("-r")
        .arg("-C")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed co start cowsay");

    if let Some(stdin) = cowsay.stdin.as_mut() {
        stdin.write_all(anek.as_bytes())
            .expect("Failed to write to stdin");
    }
    Ok(())
}
