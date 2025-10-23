use std::fs::File;
use std::io::{self, BufRead};

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


fn main() -> io::Result<()>
{
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut nr_bytes: usize;
    let mut max_bytes: usize = 0;
    let mut max_chars: usize = 0; 
    let mut string_max_bytes: String = String::new();
    let mut string_max_chars: String = String::new();
    for line_res in reader.lines() 
    {
        let line = line_res?;       
        let lungime = line.chars().count();
        nr_bytes = 0;      
        for c in line.chars() 
        {
            nr_bytes += c.len_utf8(); 
        }
        if nr_bytes > max_bytes 
        {
            max_bytes = nr_bytes;
            string_max_bytes = line.clone();
        }

        if lungime > max_chars
        {
            max_chars = lungime;
            string_max_chars = line.clone();
        }
    }

    println!("Problema 1:");
    println!("The longest line in numbers of bytes is : {string_max_bytes}");
    println!("The longest line in numbers of characters is : {string_max_chars}");

    println!("Problema 2:");
    let mut output: String = String :: new();
    let mut string_problema_2: String = String::from("azbcA.!ZBCGd");
    for c in string_problema_2.chars()
    {
        match rot13_char(c) {
                Ok(rc) => output.push(rc),
                Err(err) => {
                    eprintln!("{}", err);
                    return Ok(()); 
                }
            }
    }
    println!("{output}");
    output.clear();

    string_problema_2 = String::from("azbc🍉ZBCGd");
    for c in string_problema_2.chars()
    {
        match rot13_char(c) {
                Ok(rc) => output.push(rc),
                Err(err) => {
                    eprintln!("{}", err);
                    return Ok(()); 
                }
            }
    }
    println!("{output}");
    output.clear();


    Ok(())
}
