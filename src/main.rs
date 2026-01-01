use anyhow::{Result, anyhow};
use pulldown_cmark::html::push_html;
use pulldown_cmark::{Options, Parser};
use stdin_or_clipboard::sync::get_text_from_stdin_or_clipboard;

fn main() -> Result<()> {
    let (text, clipboard) = get_text_from_stdin_or_clipboard()
        .map_err(|e| anyhow!("failed to get text from stdin or clipboard: {e}"))?;

    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let parser = Parser::new_ext(&text, options);

    let mut html_output = String::with_capacity(text.len() * 2);
    push_html(&mut html_output, parser);

    if let Some(mut clipboard) = clipboard
        && clipboard.set_html(&html_output, Some(&text)).is_ok()
    {
        println!("{text}");
        return Ok(());
    }

    eprintln!("Oops. Failed to access the clipboard. Dumping the HTML to stdout instead:");
    println!("{html_output}");

    Ok(())
}
