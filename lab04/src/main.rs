use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn rot13_char(c: char) -> Result<char, String> {
    if !c.is_ascii() {
        return Err(format!("Error: Non-ASCII character encountered: '{}'", c));
    }

    let rotated = match c {
        'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
        'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
        _ => c,
    };

    Ok(rotated)
}

fn problema_2(string: String) -> Result<String, String> {
    let mut output = String::new();

    for c in string.chars() {
        match rot13_char(c) {
            Ok(rc) => output.push(rc),
            Err(err) => {
                eprintln!("{}", err);
                return Err(err);
            }
        }
    }

    Ok(output)
}

fn problema_3(string: String) -> String {
    let dict: HashMap<&'static str, &'static str> = HashMap::from([
        ("pt", "pentru"),
        ("ptr", "pentru"),
        ("dl", "domnul"),
        ("dna", "doamna"),
    ]);

    string
        .split_whitespace()
        .map(|w| dict.get(w).copied().unwrap_or(w))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut nr_bytes: usize;
    let mut max_bytes: usize = 0;
    let mut max_chars: usize = 0;
    let mut string_max_bytes: String = String::new();
    let mut string_max_chars: String = String::new();
    for line_res in reader.lines() {
        let line = line_res?;
        let lungime = line.chars().count();
        nr_bytes = 0;
        for c in line.chars() {
            nr_bytes += c.len_utf8();
        }
        if nr_bytes > max_bytes {
            max_bytes = nr_bytes;
            string_max_bytes = line.clone();
        }

        if lungime > max_chars {
            max_chars = lungime;
            string_max_chars = line.clone();
        }
    }

    println!("Problema 1:");
    println!("The longest line in numbers of bytes is : {string_max_bytes}");
    println!("The longest line in numbers of characters is : {string_max_chars}");

    println!("Problema 2:");
    let mut string_problema_2: String = String::from("azbcA.!ZBCGd");
    match problema_2(string_problema_2) {
        Ok(rotated) => println!("ROT13 result: {}", rotated),
        Err(_) => println!("ROT13 failed due to non-ASCII input."),
    }

    string_problema_2 = String::from("azbcA.👀!ZBCGd");
    match problema_2(string_problema_2) {
        Ok(rotated) => println!("ROT13 result: {}", rotated),
        Err(_) => println!("ROT13 failed due to non-ASCII input."),
    }

    println!("Problema 3:");
    let mut string_problema_3: String =
        String::from("Am fost la dl Matei pt că m-a invitat cu o zi înainte");
    let mut result = problema_3(string_problema_3);
    println!("{result}");

    string_problema_3 = String::from("Am fost la dna Raluca ptr a invata la Rust");
    result = problema_3(string_problema_3);
    println!("{result}");

    let path = "/etc/hosts";

    let file = File::open(Path::new(path))?;
    let reader = io::BufReader::new(file);

    for line_res in reader.lines() {
        let line = line_res?;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let mut parts = trimmed.split_whitespace();

        if let (Some(ip), Some(host)) = (parts.next(), parts.next()) {
            println!("{} => {}", host, ip);
        }
    }

    Ok(())
}
