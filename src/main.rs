use anyhow::{Result, anyhow};
use pulldown_cmark::{Options, Parser, html::push_html};
use stdin_or_clipboard::get_text_from_stdin_or_clipboard;

const OPTS: Options = Options::ENABLE_TABLES
    .union(Options::ENABLE_FOOTNOTES)
    .union(Options::ENABLE_STRIKETHROUGH)
    .union(Options::ENABLE_TASKLISTS);

fn main() -> Result<()> {
    let (text, cb) =
        get_text_from_stdin_or_clipboard().map_err(|e| anyhow!("failed to get text: {e}"))?;

    let mut html = String::with_capacity(text.len() * 2);
    push_html(&mut html, Parser::new_ext(&text, OPTS));

    if let Some(mut c) = cb
        && c.set_html(&html, Some(&text)).is_ok()
    {
        println!("{text}");
    } else {
        eprintln!("Clipboard unavailable:");
        println!("{html}");
    }
    Ok(())
}
