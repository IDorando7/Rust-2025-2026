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
    if n % 2 == 0 {
        return n == 2;
    }

    let mut d: u32 = 3;
    while d.saturating_mul(d) <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn main()
{
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
}