use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;

#[derive(Debug)]
#[allow(dead_code)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn main() -> io::Result<()> {

    let path = "students.txt";

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut students: Vec<Student> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.trim().split(',').collect();

        if parts.len() == 3 {
            let name = parts[0].to_string();
            let phone = parts[1].to_string();
            let age: u32 = parts[2].parse().unwrap_or(0);

            students.push(Student { name, phone, age });
        }
    }

    if let (Some(oldest), Some(youngest)) = (
        students.iter().max_by_key(|s| s.age),
        students.iter().min_by_key(|s| s.age),
    ) {
        println!("Oldest student: {} ({} years)", oldest.name, oldest.age);
        println!("Youngest student: {} ({} years)", youngest.name, youngest.age);
    } else {
        println!("No valid student data found.");
    }

    Ok(())
}
