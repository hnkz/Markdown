extern crate markdown;

use markdown::object::MDObject;
use markdown::Markdown;

use std::{fs};
use std::io::{
    BufReader,
    BufRead,
};

fn main() {
    let mut buf: String = String::new();
    let reader = BufReader::new(fs::File::open("testcase/test.md").expect("File does not exist."));

    for line in reader.lines() {
        match line {
            Ok(line) => {
                buf.push_str(&line);
                buf.push('\n');
            }
            Err(err) => {
                println!("lines error: {:?}", err);
                return;
            }
        }
    }

    let mut markdown = Markdown::new(buf.as_mut_str());
    markdown.output();
}
