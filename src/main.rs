use std::{env::args, process::exit};

use anyhow::{anyhow, Result};
use markdown::{to_html_with_options, Options};
use stdin_or_clipboard::sync::get_text_from_stdin_or_clipboard;

fn main() -> Result<()> {
    args().skip(1).for_each(|arg| {
        if arg.to_lowercase().contains("-V") {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            exit(0);
        } else {
            eprintln!("Unknown argument: {arg}");
            exit(1);
        }
    });

    let (text, clipboard) = get_text_from_stdin_or_clipboard()
        .map_err(|e| anyhow!("failed to get text from stdin or clipboard: {e}"))?;

    let html = to_html_with_options(&text, &Options::gfm())
        .map_err(|e| anyhow!("failed to convert markdown to HTML: {e}"))?;

    if let Some(mut clipboard) = clipboard {
        if clipboard.set_html(&html, Some(&text)).is_ok() {
            println!("{text}");
            return Ok(());
        }
    }

    eprintln!("Oops. Failed to access the clipboard. Dumping the HTML to stdout instead:");
    println!("{html}");

    Ok(())
}
