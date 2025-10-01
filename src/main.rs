fn is_prime(x: i32) -> bool
{
    let mut i: i32 = 3;

    if x <= 1 
    {
        return false;
    }

    if x == 2
    {
        return true;
    }

    if x % 2 == 0 
    {
        return false;
    }

    loop 
    {
        if i * i > x
        {
            break;
        } 

        if x % i == 0
        {
            return false;
        }

        i += 2;
    }
    return true;
}

fn are_coprime(mut x: i32, mut y: i32) -> bool
{
    let mut r: i32;
    while y > 0
    {
        r = x % y;
        x = y;
        y = r;
    }
    
    if x == 1 { true } else { false } 
}

fn first_problem()
{
    let mut x: i32 = 0;
    while x <= 100 
    {
        if is_prime(x) == true
        {
            println!("{} ", x);
        }
        x += 1;
    }
}

fn second_problem() 
{
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while x <= 100
    {
        while y <= 100
        {
            if are_coprime(x, y) == true
            {
                println!("{} and {} are coprime", x, y);
            }
            else 
            {
                println!("{} and {} are not coprime", x, y);
            }
            y += 1;
        }
        x += 1;
        y = 0;
    }
}

fn third_problem()
{
    let mut x: i32 = 99;
    while x >= 1 
    {
        println!("{} bottles of beer on the wall,", x);
        println!("{} bottles of beer.", x);
        println!("Take one down, pass it around,");

        x -= 1;
    }

    println!("No bottles of beer on the wall.");
}

fn main() 
{
    //first_problem();
    //second_problem();
    third_problem();
}

