use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, serde::Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

type Canvas = [[u8; 100]; 55];

fn new_canvas() -> Canvas {
    [[b' '; 100]; 55]
}

fn set_pixels(c: &mut Canvas, ops: &[(usize, usize, u8)]) {
    for &(x, y, val) in ops {
        if x < 55 && y < 100 {
            c[x][y] = val;
        }
    }
}

fn print(canvas: Canvas) {
    for row in &canvas {
        let mut line = String::with_capacity(100);
        for &b in row {
            line.push(b as char);
        }
        println!("{line}");
    }
}

fn main() -> io::Result<()> {
    println!("Problema 1");
    let path = "students.txt";

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Could not open {}: {}", path, e),
    };
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
        println!(
            "Youngest student: {} ({} years)",
            youngest.name, youngest.age
        );
    } else {
        println!("No valid student data found.");
    }

    println!("\nProblema 3 (JSON lines)");
    let json_path = "students_json.txt";

    let file = match File::open(json_path) {
        Ok(f) => f,
        Err(e) => panic!("Could not open {}: {}", json_path, e),
    };
    let reader = io::BufReader::new(file);

    let mut students_json: Vec<Student> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match serde_json::from_str::<Student>(line) {
            Ok(st) => students_json.push(st),
            Err(e) => eprintln!(" Skipping malformed JSON line:\n  {line}\n  → {e}"),
        }
    }

    if let (Some(oldest), Some(youngest)) = (
        students_json.iter().max_by_key(|s| s.age),
        students_json.iter().min_by_key(|s| s.age),
    ) {
        println!(
            "Oldest student: {} ({} years, phone: {})",
            oldest.name, oldest.age, oldest.phone
        );
        println!(
            "Youngest student: {} ({} years, phone: {})",
            youngest.name, youngest.age, youngest.phone
        );
    } else {
        println!("No valid student data found in JSON file.");
    }

    println!("\nProblema 2");

    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(
        c,
        &[
            (2, 16, 95),
            (5, 35, 92),
            (6, 3, 95),
            (2, 11, 95),
            (5, 3, 95),
        ],
    );
    set_pixels(
        c,
        &[
            (2, 38, 95),
            (4, 9, 40),
            (3, 41, 124),
            (2, 37, 95),
            (2, 25, 124),
        ],
    );
    set_pixels(
        c,
        &[
            (5, 27, 124),
            (2, 27, 124),
            (4, 0, 124),
            (3, 35, 47),
            (2, 18, 95),
        ],
    );
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(
        c,
        &[
            (6, 30, 95),
            (5, 45, 95),
            (4, 31, 124),
            (4, 7, 124),
            (3, 43, 39),
        ],
    );
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(
        c,
        &[
            (3, 44, 95),
            (3, 19, 92),
            (5, 23, 95),
            (3, 8, 47),
            (2, 10, 95),
        ],
    );
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 10, 95),
            (4, 44, 95),
            (2, 9, 95),
            (0, 32, 95),
            (5, 2, 95),
        ],
    );
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(
        c,
        &[
            (3, 46, 92),
            (5, 25, 44),
            (1, 43, 124),
            (5, 46, 47),
            (3, 15, 47),
        ],
    );
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(
        c,
        &[
            (5, 44, 95),
            (2, 2, 95),
            (5, 10, 95),
            (5, 9, 95),
            (4, 43, 124),
        ],
    );
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(
        c,
        &[
            (4, 40, 124),
            (3, 3, 95),
            (4, 4, 124),
            (6, 31, 47),
            (3, 4, 96),
        ],
    );
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 12, 92),
            (2, 32, 95),
            (5, 37, 95),
            (5, 26, 95),
            (5, 39, 47),
        ],
    );
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(
        c,
        &[
            (5, 36, 95),
            (7, 30, 95),
            (6, 4, 47),
            (4, 24, 95),
            (1, 32, 95),
        ],
    );
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(canvas);

    Ok(())
}
