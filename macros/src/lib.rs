#![feature(proc_macro_span)]

extern crate proc_macro;
use std::{process::Command};

use proc_macro::{LineColumn, TokenStream, TokenTree};
use quote::quote;

struct Source {
    source: String,
    line: usize,
    col: usize,
}

impl Source {
    fn reconstruct_from(&mut self, input: TokenStream) {
        for t in input {
            if let TokenTree::Group(g) = t {
                let s = g.to_string();
                self.add_whitespace(g.span_open().start());
                self.add_str(&s[..1]); // the '[', '{' or '('.
                self.reconstruct_from(g.stream());
                self.add_whitespace(g.span_close().start());
                self.add_str(&s[s.len() - 1..]); // the ']', '}' or ')'.
            } else {
                self.add_whitespace(t.span().start());
                self.add_str(&t.to_string());
            }
        }
    }

    fn add_str(&mut self, s: &str) {
        // Let's assume for now s contains no newlines.
        self.source += s;
        self.col += s.len();
    }

    fn add_whitespace(&mut self, loc: LineColumn) {
        while self.line < loc.line {
            self.source.push('\n');
            self.line += 1;
            self.col = 0;
        }
        while self.col < loc.column {
            self.source.push(' ');
            self.col += 1;
        }
    }
}

fn run_applescript(script: &str) {
    let process = Command::new("osascript")
        .arg("-l")
        .arg("AppleScript")
        .arg("-e")
        .arg(script)
        .spawn()
        .expect("Failed to run AppleScript");

    let _ = process.wait_with_output().unwrap();
}

/// Run AppleScript
#[proc_macro]
pub fn applescript(input: TokenStream) -> TokenStream {
    let mut s = Source {
        source: String::new(),
        line: 1,
        col: 0,
    };
    s.reconstruct_from(input);
    println!("{}", s.source);

    let source = s.source;

    run_applescript(&source);
    quote!().into()
}
