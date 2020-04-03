extern crate clap;
extern crate rand;

use clap::{App, Arg};
use rand::Rng;
use std::str::FromStr;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn create_line(max_words: i16, word_list: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let mut line: Vec<String> = Vec::new();
    for _ in 1..max_words + 1 {
        let random_line = rng.gen_range(0, word_list.len() + 1);
        line.push(word_list[random_line].to_string())
    }
    line.join(" ")
}

fn init_word_list(list_path: &str) -> Vec<String> {
    let mut word_list: Vec<String> = Vec::new();
    let lines = lines_from_file(list_path);
    for line in lines {
        word_list.push(line);
    }
    word_list
}

fn main() {
    let matches = App::new("dadavanga")
        .version("1.0")
        .author("Burak OZ <burakoz@zoho.com>")
        .about("A cli app for dadaist poetry")
        .arg(
            Arg::with_name("path")
                .help("Path of your wordlist")
                .short("f")
                .long("file")
                .value_name("file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("lines")
                .help("Max lines for poem | DEFAULT = 4")
                .short("l")
                .long("lines")
                .value_name("lines")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("words_per_line")
                .help("Max words value per line | DEFAULT = 5")
                .short("w")
                .long("words")
                .value_name("words")
                .takes_value(true),
        )
        .get_matches();

    let max_lines: usize = FromStr::from_str(matches.value_of("lines").unwrap_or("4")).unwrap();
    let max_words = FromStr::from_str(matches.value_of("words_per_line").unwrap_or("5")).unwrap();
    let file_path = matches.value_of("file").unwrap_or("./list.txt");

    let word_list = init_word_list(file_path);

    for _ in 1..max_lines + 1 {
        let line = create_line(max_words, &word_list);
        println!("{}", line);
    }
}
