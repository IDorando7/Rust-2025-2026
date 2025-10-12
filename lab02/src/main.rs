fn add_space(s: &mut String, x: i32) {
    let mut i: i32 = 1;
    while i <= x {
        s.push(' ');
        i += 1;
    }
}

fn add_str(mut s: String, c: &str) -> String {
    s += c;
    s
}

fn add_integer(s: &mut String, mut x: i32) {
    let mut string_number = String::from("");
    while x > 0 {
        let d = (((x % 10) as u8) + b'0') as char;
        string_number.push(d);
        x /= 10;
    }

    let reversed: String = string_number.chars().rev().collect();
    let mut i: i32 = 0;
    while i < reversed.len() as i32 {
        if i % 3 == 0 && i != 0 {
            s.push('_');
        }

        s.push_str(&reversed[i as usize..(i + 1) as usize]);
        i += 1;
    }
}

fn add_float(s: &mut String, x: f32) {
    let intreg: i32 = x.trunc() as i32;
    add_integer(s, intreg);
    s.push('.');

    let mut fractionar_string: String = String::new();
    let mut i = 0;
    let mut fractionar: f32 = x.fract();

    while i < 3 {
        fractionar *= 10.0;
        let d = fractionar.trunc() as u8;
        fractionar_string.push((b'0' + d) as char);
        fractionar -= d as f32;
        i += 1;
    }

    s.push_str(&fractionar_string);
}

fn add_chars_n(mut s: String, c: char, x: i32) -> String {
    let mut i: i32 = 1;
    while i <= x {
        s.push(c);
        i += 1;
    }

    s
}

fn add_chars_n_reference(s: &mut String, c: char, x: i32) {
    let mut i: i32 = 1;
    while i <= x {
        s.push(c);
        i += 1;
    }
}
fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    println!("{}\n", s);
    s.clear();

    i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_reference(&mut s, c, 26 - i);

        i += 1;
    }
    println!("{}\n", s);
    s.clear();

    add_space(&mut s, 49);
    s = add_str(s, "I");
    add_space(&mut s, 1);
    s = add_str(s, "💚");
    println!("{s}");
    s.clear();

    add_space(&mut s, 49);
    s = add_str(s, "RUST.");
    println!("{s}");

    s.clear();
    add_space(&mut s, 15);
    s = add_str(s, "Most");
    add_space(&mut s, 10);
    s = add_str(s, "crate");
    add_space(&mut s, 3);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 9);
    s = add_str(s, "and");
    add_space(&mut s, 3);
    s = add_str(s, "latest");
    add_space(&mut s, 7);
    s = add_str(s, "is");
    println!("{s}");

    s.clear();
    add_space(&mut s, 15);
    add_space(&mut s, 4);
    s = add_str(s, "downloaded");
    add_space(&mut s, 5);
    s = add_str(s, "has");
    add_space(&mut s, 11);
    s = add_str(s, "downloads");
    add_space(&mut s, 3);
    s = add_str(s, "the");
    add_space(&mut s, 6);
    s = add_str(s, "version");
    add_space(&mut s, 3);
    add_float(&mut s, 2.038);
    println!("{s}");
}
