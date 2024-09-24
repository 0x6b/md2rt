use anyhow::{anyhow, Result};
use clap::Parser;
use markdown::{to_html_with_options, Options};
use stdin_or_clipboard::sync::get_text_from_stdin_or_clipboard;

// Just to implement `--help` and `--version` flags.
#[derive(Parser)]
#[clap(version, about)]
struct Args {}

fn main() -> Result<()> {
    Args::parse();

    let (text, clipboard) = get_text_from_stdin_or_clipboard()
        .map_err(|e| anyhow!("failed to get text from stdin or clipboard: {e}"))?;

    if clipboard.is_none() {
        Err(anyhow!("no clipboard available"))?;
    }

    let html = to_html_with_options(&text, &Options::gfm())
        .map_err(|e| anyhow!("failed to convert markdown to HTML: {e}"))?;

    println!("{text}");

    clipboard
        .unwrap()
        .set_html(html, Some(text.to_string()))
        .map_err(|e| anyhow!("failed to set HTML to clipboard: {e}"))
}
