use std::io::{stdin, IsTerminal, Read};

use anyhow::{anyhow, Result};
use arboard::Clipboard;
use clap::Parser;
use markdown::{to_html_with_options, Options};

// Just to implement `--help` and `--version` flags.
#[derive(Parser)]
#[clap(version, about)]
struct Args {}

fn main() -> Result<()> {
    Args::parse();

    let mut clipboard =
        Clipboard::new().map_err(|e| anyhow!("failed to access system clipboard: {e}"))?;

    // If the program is run in a terminal, read from the clipboard. Otherwise, read from stdin.
    let mut text = String::new();
    if stdin().is_terminal() {
        text = clipboard
            .get_text()
            .map_err(|e| anyhow!("failed to get text from clipboard: {e}"))?;
    } else {
        stdin()
            .lock()
            .read_to_string(&mut text)
            .map_err(|e| anyhow!("failed to read from stdin: {e}"))?;
    }

    let html = to_html_with_options(&text, &Options::gfm())
        .map_err(|e| anyhow!("failed to convert markdown to HTML: {e}"))?;

    println!("{text}");
    clipboard
        .set_html(html, Some(text.to_string()))
        .map_err(|e| anyhow!("failed to set HTML to clipboard: {e}"))
}
