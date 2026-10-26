// 1
fn next_prime(x: u16) -> Option<u16> {
    'get_prime: for i in (x + 1)..u16::MAX {
        for p in 2..(i / 2) {
            if i % p == 0 {
                continue 'get_prime;
            }
        }
        return Some(i);
    }
    None
}
fn main1() {
    let mut val = next_prime(2);
    while val.is_some() {
        let value = val.unwrap_or(u16::MAX);
        println!("{}", value);
        val = next_prime(value);
    }
    println!("None encountered")
}
// 2
fn my_add(x: u32, y: u32) -> u32 {
    let x = x as u64;
    let y = y as u64;
    if x + y >= u32::MAX as u64 {
        panic!("value overflow")
    }
    (x + y) as u32
}
fn my_mul(x: u32, y: u32) -> u32 {
    let x = x as u64;
    let y = y as u64;
    if x * y >= u32::MAX as u64 {
        panic!("value overflow")
    }
    (x * y) as u32
}

fn main2() {
    println!("{}", my_add(u32::MAX / 2, u32::MAX / 2));
    println!("{}", my_mul(u32::MAX / 2, u32::MAX / 2));
}
#[derive(Debug)]
enum MyError {
    ValueOverflow,
}
fn my_add2(x: u32, y: u32) -> Result<u32, MyError> {
    let x = x as u64;
    let y = y as u64;
    if x + y >= u32::MAX as u64 {
        return Err(MyError::ValueOverflow);
    }
    Ok((x + y) as u32)
}

fn my_mul2(x: u32, y: u32) -> Result<u32, MyError> {
    let x = x as u64;
    let y = y as u64;
    if x * y >= u32::MAX as u64 {
        return Err(MyError::ValueOverflow);
    }
    Ok((x * y) as u32)
}

fn main3() {
    let v = my_add2(u32::MAX / 2, u32::MAX / 2);
    match v {
        Ok(x) => {
            println!("{}", x)
        }
        Err(y) => {
            println!("Error: {:?}", y);
        }
    }

    let v2 = my_mul2(u32::MAX / 2, u32::MAX / 2);
    match v2 {
        Ok(x) => {
            println!("{}", x)
        }
        Err(y) => {
            println!("Error: {:?}", y);
        }
    }
}

//4
#[derive(Debug)]
enum Errors {
    CharNotASCII,
    CharNotDigit,
    CharNotBase16,
    CharNotLetter,
    CharNotPrintable,
}

fn to_uppercase(c: char) -> Result<char, Errors> {
    if !c.is_ascii() {
        return Err(Errors::CharNotASCII);
    }
    if c.is_numeric() || c.is_ascii_punctuation() {
        return Err(Errors::CharNotLetter);
    }
    Ok(c.to_ascii_uppercase())
}

fn to_lowercase(c: char) -> Result<char, Errors> {
    if !c.is_ascii() {
        return Err(Errors::CharNotASCII);
    }
    if c.is_numeric() || c.is_ascii_punctuation() {
        return Err(Errors::CharNotLetter);
    }
    Ok(c.to_ascii_lowercase())
}

fn print_char(c: char) -> Result<(), Errors> {
    if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() {
        println!("{}", c)
    } else {
        return Err(Errors::CharNotPrintable);
    }
    Ok(())
}

fn char_to_number(c: char) -> Result<u32, Errors> {
    if !c.is_numeric() {
        return Err(Errors::CharNotDigit);
    }
    Ok(c.to_digit(10).unwrap_or(0))
}

fn char_to_number_hex(c: char) -> Result<u32, Errors> {
    if !c.is_ascii_hexdigit() {
        return Err(Errors::CharNotBase16);
    }
    Ok(c.to_digit(16).unwrap_or(0))
}

fn main4() {
    println!("{}", to_lowercase('A').unwrap_or('_'));
    println!("{:?}", to_lowercase('#').err());

    println!("{}", to_uppercase('a').unwrap_or('_'));
    println!("{:?}", to_uppercase('#').err());

    match print_char('ă') {
        Ok(()) => {}
        Err(x) => {
            println!("{:?}", x);
        }
    }

    println!("{}", char_to_number('9').unwrap_or(0));
    println!("{:?}", char_to_number('#').err());

    println!("{}", char_to_number_hex('9').unwrap_or(0));
    println!("{:?}", char_to_number_hex('#').err());
}

//5
#[derive(Debug)]
enum NewError {
    Bleh,
}
fn do_something(x: i32) -> Result<i32, NewError> {
    if x % 2 == 0 {
        return Ok(x * 2 + 1);
    }
    Err(NewError::Bleh)
}
fn main5() {
    for i in 1..16 {
        match do_something(i) {
            Ok(x) => {
                println!("{}", x);
            }
            Err(y) => {
                println!("Error: {:?}", y)
            }
        }
    }
}

fn main() {
    main1();
    main2();
    main3();
    main4();
    main5();
}
