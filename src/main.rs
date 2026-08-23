use std::{
    fs,
    io::{self, Read, Write},
    process,
};

use clap::Parser;
use regex::Regex;

/// Strip comments, replace -- with mdash, and pull !/? inside adjacent italic runs
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
    let content = strip_and_replace(&content);

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

fn strip_and_replace(s: &str) -> String {
    let comment = Regex::new(r"(?s)<!--.+?-->").unwrap();
    let emph_star = Regex::new(r"\*([^*]+)\*([!?])").unwrap();
    let content = comment.replace_all(s, "");
    let content = emph_star.replace_all(&content, "*${1}${2}*");
    content.replace("--", "—")
}

#[cfg(test)]
mod test {
    #[test]
    fn strip_and_replace_works() {
        let sample = include_str!("../resource/sample.md");
        let expected = include_str!("../resource/clean-sample.md");
        let actual = super::strip_and_replace(sample);
        assert_eq!(actual, expected);
    }

    #[test]
    fn moves_tall_punctuation_into_italics() {
        assert_eq!(super::strip_and_replace("Sit *down*!"), "Sit *down!*");
        assert_eq!(super::strip_and_replace("*What*?"), "*What?*");
    }

    #[test]
    fn leaves_low_punctuation_and_bold_alone() {
        assert_eq!(
            super::strip_and_replace("He sat *down*, shrugged."),
            "He sat *down*, shrugged."
        );
        assert_eq!(
            super::strip_and_replace("It went **boom**!"),
            "It went **boom**!"
        );
    }
}
