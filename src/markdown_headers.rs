use anyhow::{anyhow, Result};
use colored::*;
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;

/// Read and print headers of a markdown-formatted file
pub fn print_markdown_headers(path: &Path) -> Result<()> {
    let headers = read_headers(path)?;
    print_headers(&headers);
    Ok(())
}

/// Representation of a header in a "markdown"-file
#[derive(Debug)]
struct Header {
    level: u8,
    text: String,
}

impl Header {
    fn new(level: u8, text: &str) -> Self {
        Header {
            level,
            text: text.to_string(),
        }
    }
}

/// Read file into list of [Header]
///
/// The file is parsed as a markdown and all headers are filtered out and returned
fn read_headers(path: &Path) -> Result<Vec<Header>> {
    if !path.exists() {
        return Err(anyhow!("File does not exist: {:?}", path));
    }
    let txt = read_to_string(path)?;
    let html = markdown::to_html(&txt);
    let re = Regex::new(r"<h([1-9])>(.*)</h[1-9]>").unwrap();
    Ok(re
        .captures_iter(&html)
        .map(|x| {
            Header::new(
                x.get(1).unwrap().as_str().parse().unwrap(),
                x.get(2).unwrap().as_str(),
            )
        })
        .collect())
}

/// Print list of markdown headers
///
/// Uses different colors for each header level and indents each level
fn print_headers(headers: &[Header]) {
    let level_to_color = |l: u8| -> Color {
        match l {
            1 => Color::Yellow,
            2 => Color::Red,
            3 => Color::Blue,
            4 => Color::Green,
            _ => Color::Black,
        }
    };

    for x in headers {
        let space = if x.level == 1 {
            0
        } else {
            ((x.level as usize) - 2) * 2
        };
        println!("{:space$}{}", "", x.text.color(level_to_color(x.level)),);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn read_headers() {
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let headers =
            super::read_headers(&base.join("examples/example.md")).expect("error reading headers");
        assert!(headers.len() > 0, "no headers created");
    }
}
