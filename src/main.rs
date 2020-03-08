use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::LinesWithEndings;

use std::io::stdin;
use std::io::Read;
use std::fmt::Write;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let ftype = args.get(1).unwrap();

    let mut s = String::new();
    stdin().read_to_string(&mut s).expect("Could not read strng!");
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(&ftype).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["InspiredGitHub"]);
    for line in LinesWithEndings::from(&s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_satysfi_escaped(&ranges[..]);
        print!("{}", escaped);
    }
}

fn as_satysfi_escaped(v: &[(Style, &str)]) -> String {
    let mut s: String = String::new();
    // let mut prev_style: Option<Style> = None;
    s.push('|');
    s.push(' ');

    fn fg(style: &Style, text: &str) -> String {
        let back_quote = match text {
            text if text.contains("```") => "````",
            text if text.contains("``") => "```",
            text if text.contains("`") => "``",
            _ => "`"
        };
        let begin = match text {
            text if Some('`') == text.chars().nth(0)
                => format!("{} ", back_quote),
            _ => format!("#{}", back_quote)
        };
        let end = match text {
            text if Some('`') == text.chars().rev().nth(0)
                => format!(" {}", back_quote),
            _ => format!("{}#", back_quote)
        };
        let quoted = match text {
            "" => String::from("` `"),
            text => format!("{}{}{}", begin, text, end)
        };
        format!(
            "\\fg(({},{},{}))({});",
            style.foreground.r,
            style.foreground.g,
            style.foreground.b,
            quoted
        )
    }

    for &(style, text) in v.iter() {
        let text = text.trim_end_matches('\n');
        write!(s, "{}", fg(&style, text)).expect("Failed to write satysfi cmd!");
    }
    s.push('\n');
    s
}
