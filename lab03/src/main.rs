use  std :: panic;
use std::io::{self, Write};

#[allow(dead_code)]
#[derive(Debug)]
enum ArithError {
    AddOverflow { a: u32, b: u32 },
    MulOverflow { a: u32, b: u32 },
}
enum FunctionError {
    Ascii,
    Digit,
    HexDigit,
    Letter,
    Printable,
}

fn to_uppercase(ch: char) -> Result<char, (FunctionError, char)>
{
    match ch {
        'a'..='z' => Ok(((ch as u8) - (b'a' - b'A')) as char),
        'A'..='Z' => Ok(ch),
        _ => Err((FunctionError::Letter, ch)),
    }
}

fn to_lowercase(ch: char) -> Result<char, (FunctionError, char)> {
    match ch {
        'a'..='z' => Ok(ch),
        'A'..='Z' => Ok(((ch as u8) + (b'a' - b'A')) as char),
        _ => Err((FunctionError::Letter, ch)),
    }
}

fn print_char(ch: char) -> Result<char, (FunctionError, char)> {
    match ch.is_ascii_graphic() {
        true => Ok(ch),
        false => Err((FunctionError::Printable, ch)),
    }
}

fn char_to_number(ch: char) -> Result<u8, (FunctionError, char)> {
    if !ch.is_ascii() {
        return Err((FunctionError::Ascii, ch));
    }
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        _ => Err((FunctionError::Digit, ch)),
    }
}

fn char_to_number_hex(ch: char) -> Result<u8, (FunctionError, char)> {
    if !ch.is_ascii() {
        return Err((FunctionError::Ascii, ch));
    }
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        'A'..='F' => Ok((ch as u8 - b'A') + 10u8),
        _ => Err((FunctionError::HexDigit, ch)),
    }
}

fn print_error(eroare: (FunctionError, char)) {
    match eroare.0 {
        FunctionError::Ascii => println!("The characther '{}' is not ASCII.", eroare.1),
        FunctionError::Digit => println!("The characther '{}' is not a digit", eroare.1),
        FunctionError::HexDigit => println!("The characther '{}' is not a base 16 digit.", eroare.1),
        FunctionError::Letter => println!("The characther '{}' is not a letter.", eroare.1),
        FunctionError::Printable => println!("The characther {:?} is not printable.", eroare.1),
    }
}

fn next_prime(x: u16) -> Option<u16> {

    let mut cand: u32 = x as u32 + 1;
    let max: u32 = u16::MAX as u32;

    while cand <= max {
        if is_prime_u32(cand) {
            return Some(cand as u16);
        }
        cand += 1;
    }
    None
}

fn is_prime_u32(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n.is_multiple_of(2) {
        return n == 2;
    }

    let mut d: u32 = 3;
    while d.saturating_mul(d) <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 2;
    }
    true
}

fn checked_add_u32(a: u32, b: u32) -> u32 
{
    if u32::MAX - a < b { panic!("overflow: {a} + {b}"); }
    a + b
}

fn checked_mul_u32(a: u32, b: u32) -> u32 
{
    if a != 0 && b != 0 && a > u32::MAX / b { panic!("overflow: {a} * {b}"); }
    a * b
}

fn checked_add_u32_result(a: u32, b: u32) -> Result<u32, ArithError> 
{
    if u32::MAX - a < b { Err(ArithError::AddOverflow { a, b }) } else { Ok(a + b) }
}

fn checked_mul_u32_result(a: u32, b: u32) -> Result<u32, ArithError>
{
    if a != 0 && a > u32::MAX / b { Err(ArithError::MulOverflow { a, b }) } else { Ok(a * b) }
}

fn add_then_mul(a: u32, b: u32, c: u32) -> Result<u32, ArithError>
{
    let sum = checked_add_u32_result(a, b)?;
    checked_mul_u32_result(sum, c)      
}


fn main()
{
    //-------------------------------------PROBLEMA1----------------------------------------

    println!("Problem 1 tests:");

    let good_number: u16 = 25u16;
    let bad_number: u16 = u16::MAX;

    match next_prime(good_number) {
        Some(x) => println!("The next prime after {good_number} is {x}"),
        None => println!("There isn't a prime after {good_number} that fits in u16."),
    };

    match next_prime(bad_number) {
        Some(x) => println!("The next prime after {bad_number} is {x}"),
        None => println!("There isn't a prime after {bad_number} that fits in u16."),
    };

    //-------------------------------------PROBLEMA2----------------------------------------

    println!("Problem 2 tests:");

    let a = 35u32;
    let b = 25u32;
    let c = u32::MAX;

    let result = panic::catch_unwind(|| checked_add_u32(a, b));
    match result {
        Ok(res) => println!("The sum of {a} and {b} is {res:?}"),
        Err(_) => println!("Program panicked. The sum of {a} and {b} counldn't be computed."),
    }

    let result = panic::catch_unwind(|| checked_add_u32(b, c));
    match result {
        Ok(res) => println!("The sum of {b} and {c} is {res:?}"),
        Err(_) => println!("Program panicked. The sum of {b} and {c} counldn't be computed."),
    }

    let result = panic::catch_unwind(|| checked_mul_u32(a, b));
    match result {
        Ok(res) => println!("The multiplication of {a} and {b} is {res:?}."),
        Err(_) => {
            println!("Program panicked. The multiplication of {a} and {b} counldn't be computed")
        }
    }

    let result = panic::catch_unwind(|| checked_mul_u32(b, c));
    match result {
        Ok(res) => println!("The multiplication of {b} and {c} is {res:?}."),
        Err(_) => {
            println!("Program panicked. The multiplication of {b} and {c} counldn't be computed")
        }
    }

    //-------------------------------------PROBLEMA3----------------------------------------

    match add_then_mul(40, 2, 1000)
    {
        Ok(v) => println!("(40 + 2) * 1000 = {v}"),
        Err(_) => eprintln!("Propagated error"),
    }

    match add_then_mul(u32::MAX, 0, 2) {
        Ok(v) => println!("val = {v}"),
        Err(_) => eprintln!("Propagated error"),
    }

    //-------------------------------------PROBLEMA4----------------------------------------

    match to_uppercase('d') {
        Ok(c) => println!("Uppercase of 'd' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_uppercase('A') {
        Ok(c) => println!("Uppercase of 'A' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_uppercase('.') {
        Ok(c) => println!("Uppercase of '.' is '{c}'."),
        Err(e) => print_error(e),
    }

    match to_lowercase('H') {
        Ok(c) => println!("Lowercase of 'H' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_lowercase('b') {
        Ok(c) => println!("Lowercase of 'b' is '{c}'."),
        Err(e) => print_error(e),
    }
    match to_lowercase('/') {
        Ok(c) => println!("Lowercase of '/' is '{c}'."),
        Err(e) => print_error(e),
    }

    match print_char(';') {
        Ok(c) => println!("Printable char: '{c}'."),
        Err(e) => print_error(e),
    }
    match print_char('\n') {
        Ok(c) => println!("Printable char: '{c}'."),
        Err(e) => print_error(e),
    }

    match char_to_number('2') {
        Ok(c) => println!("The conversion of '2' to digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number('x') {
        Ok(c) => println!("The conversion of '2' to digit is {c}."),
        Err(e) => print_error(e),
    }

    match char_to_number_hex('D') {
        Ok(c) => println!("The conversion of 'D' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('y') {
        Ok(c) => println!("The conversion of 'y' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('8') {
        Ok(c) => println!("The conversion of '8' to base 16 digit is {c}."),
        Err(e) => print_error(e),
    }

    //-------------------------------------PROBLEMA5----------------------------------------
    //-------------------------------------CALCULATOR---------------------------------------

     #[derive(Debug)]
    enum CalcError {
        InvalidInput(String),
        DivisionByZero,
        Overflow(String),
    }

    fn read_line(prompt: &str) -> Option<String> {
        print!("{prompt}");
        io::stdout().flush().ok()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;
        let trimmed = input.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    }

    fn calculate(a: u32, b: u32, op: &str) -> Result<u32, CalcError> {
        match op {
            "add" => a.checked_add(b).ok_or(CalcError::Overflow(format!("{a} + {b}"))),
            "sub" => a.checked_sub(b).ok_or(CalcError::Overflow(format!("{a} - {b}"))),
            "mul" => a.checked_mul(b).ok_or(CalcError::Overflow(format!("{a} * {b}"))),
            "div" => {
                if b == 0 {
                    Err(CalcError::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            }
            _ => Err(CalcError::InvalidInput(op.to_string())),
        }
    }

    println!("Simple Safe Calculator (supports add, sub, mul, div)");
    println!("Type 'exit' to quit.\n");

    loop {
        let a_str = match read_line("Enter first number: ") {
            Some(s) => {
                if s.eq_ignore_ascii_case("exit") { break; }
                s
            }
            None => {
                println!("No input detected. Exiting calculator...");
                break;
            }
        };

        let a = match a_str.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid number '{}'. Try again.", a_str);
                continue;
            }
        };

        let op = match read_line("Enter operation (add/sub/mul/div): ") {
            Some(s) => s.to_ascii_lowercase(),
            None => {
                println!("No operation entered. Exiting calculator...");
                break;
            }
        };

        let b_str = match read_line("Enter second number: ") {
            Some(s) => s,
            None => {
                println!("No input detected. Exiting calculator...");
                break;
            }
        };

        let b = match b_str.parse::<u32>() {
            Ok(val) => val,
            Err(_) => {
                println!("Invalid number '{}'. Try again.", b_str);
                continue;
            }
        };

        match calculate(a, b, &op) {
            Ok(result) => println!("Result of {} {} {} = {}\n", a, op, b, result),
            Err(CalcError::InvalidInput(op)) => println!("Invalid operation '{op}'. Try again.\n"),
            Err(CalcError::DivisionByZero) => println!("Error: cannot divide by zero!\n"),
            Err(CalcError::Overflow(expr)) => println!("Overflow detected in {expr}\n"),
        }
    }

}