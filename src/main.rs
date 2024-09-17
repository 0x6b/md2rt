use std::io::{stdin, IsTerminal, Read};

use anyhow::{bail, Result};
use clap::Parser;

// Just to implement `--help` and `--version` flags.
#[derive(Parser)]
#[clap(version, about)]
struct Args {}

fn main() -> Result<()> {
    let mut c = match arboard::Clipboard::new() {
        Ok(c) => c,
        Err(why) => bail!("failed to access system clipboard: {why}"),
    };
    let _ = Args::parse();

    // If the program is run in a terminal, read from the clipboard. Otherwise, read from stdin.
    let text = if stdin().is_terminal() {
        match c.get_text() {
            Ok(t) => t.trim().to_string(),
            Err(why) => bail!("failed to get text from clipboard: {why}"),
        }
    } else {
        let mut text = String::new();
        match stdin().lock().read_to_string(&mut text) {
            Ok(_) => text,
            Err(why) => bail!("failed to read from stdin: {why}"),
        }
    };

    let html = match markdown::to_html_with_options(&text, &markdown::Options::gfm()) {
        Ok(h) => h,
        Err(why) => bail!("failed to convert markdown to HTML: {why}"),
    };

    println!("{text}");
    match c.set_html(html, Some(text)) {
        Ok(_) => Ok(()),
        Err(why) => bail!("failed to set HTML to clipboard: {why}"),
    }
}
