use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;
use rand::Rng;
#[derive(Parser, Default, Debug)]
#[clap(name = "xkcdpwgen", version = "0.1.0", author = "Carlo D'Ugo", about)]
/// A password generator using XKCD's password philosophy
struct Arguments {
    #[clap(short, long, default_value = "4")]
    words: i128,
    #[clap(short, long, default_value = "0")]
    caps: i128,
    #[clap(short, long, default_value = "0")]
    numbers: i128,
    #[clap(short, long, default_value = "0")]
    symbols: i128
}

fn get_word_list_from_file() -> Vec<String> {
    let file = File::open("words.txt").unwrap();
    let reader = BufReader::new(file);
    let mut words = Vec::new();
    for line in reader.lines() {
        words.push(line.unwrap());
    }
    words
}

fn random_words(words: &Vec<String>, n: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    words.choose_multiple(&mut rng, n).cloned().collect()
}

fn capitalize_words(words: &mut Vec<String>, n: usize) {
    let indices = (0..words.len())
        .collect::<Vec<usize>>()
        .choose_multiple(&mut rand::thread_rng(), n)
        .cloned()
        .collect::<Vec<usize>>();
    
    for i in indices {
        words[i] = capitalize_first_letter(&words[i]);
    }
}

fn capitalize_first_letter(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str()
    }
}

fn add_numbers(words: &mut Vec<String>, n: usize) {
    let indices = (0..words.len())
        .collect::<Vec<usize>>()
        .choose_multiple(&mut rand::thread_rng(), n)
        .cloned()
        .collect::<Vec<usize>>();

    for i in indices {
        let mut word = words[i].clone();
        let mut rng = rand::thread_rng();
        let number = rng.gen_range(1..10);
        let position = rng.gen_range(0..word.len());
        println!("number: {} position: {}", number, position);
        println!("word1: {}", word);
        word.insert(position, std::char::from_digit(number, 10).unwrap());
        println!("word2: {}", word);
        words[i] = word;
    }
}

fn add_symbols(words: &mut Vec<String>, n: usize) {
    let indices = (0..words.len())
        .collect::<Vec<usize>>()
        .choose_multiple(&mut rand::thread_rng(), n)
        .cloned()
        .collect::<Vec<usize>>();

    let symbols = String::from("~!@#$%^&*.:;");

    for i in indices {
        let mut word = words[i].clone();
        let mut rng = rand::thread_rng();
        let symbol_idx = rng.gen_range(0..symbols.len());
        let position = rng.gen_range(0..word.len());
        let symbol = symbols.chars().nth(symbol_idx).unwrap();
        word.insert(position, symbol);
        words[i] = word;
    }
}

fn main() {
    let args = Arguments::parse();
    let words = get_word_list_from_file();
    let mut random_words = random_words(&words, args.words as usize);
    capitalize_words(&mut random_words, args.caps as usize);
    add_numbers(&mut random_words, args.numbers as usize);
    add_symbols(&mut random_words, args.symbols as usize);
    println!("{}", random_words.join(""));
}
