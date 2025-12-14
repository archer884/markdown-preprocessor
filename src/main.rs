use std::{
    fs,
    io::{self, Read, Write},
    process,
};

use clap::Parser;
use regex::Regex;

/// Replace -- with mdash in markdown documents
#[derive(Debug, Parser)]
#[command(author, about)]
struct Opts {
    /// read from path instead of stdin
    path: Option<String>,
}

fn main() {
    if let Err(e) = run(Opts::parse()) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn run(opts: Opts) -> io::Result<()> {
    let content = load_content(&opts)?;

    let comment = Regex::new("<!--.+?-->").unwrap();
    let content = comment.replace(&content, "");
    let content = content.replace("--", "—");

    io::stdout().lock().write_all(content.as_bytes())?;

    Ok(())
}

fn load_content(opts: &Opts) -> io::Result<String> {
    if let Some(path) = opts.path.as_deref() {
        return fs::read_to_string(path);
    }

    let mut content = String::new();
    io::stdin().lock().read_to_string(&mut content)?;
    Ok(content)
}
