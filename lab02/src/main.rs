//1
fn add_chars_n(s: String, c: char, n: u32) -> String {
    let mut s = s;
    for _ in 0..n {
        s.push(c);
    }
    s
}

fn ex1() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    println!("{}", s);
}

//2
fn add_chars_n2(s: &mut String, c: char, n: u32) {
    for _ in 0..n {
        s.push(c);
    }
}

fn ex2() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n2(&mut s, c, 26 - i);

        i += 1;
    }

    println!("{}", s);
}

//3
fn add_space(s: String, n: i32) -> String {
    let mut s: String = s;
    for _ in 0..n {
        s.push(' ');
    }
    s
}

fn add_str(s: String, s2: String) -> String {
    let mut s: String = s;
    for i in s2.chars() {
        s.push(i);
    }
    s
}

fn add_integer(s: String, i: i32, sep: u8) -> String {
    if i == 0 {
        return s;
    }

    let mut s: String = s;
    s = add_integer(s, i / 10, sep + 1);
    s.push(((i % 10) as u8 + b'0') as char);
    if sep != 0 && sep.is_multiple_of(3) {
        s.push(',');
    }
    s
}

fn add_float(s: String, f: f32) -> String {
    let mut s: String = s;
    let mut f: f32 = f;

    //we pull off some funky stuff
    let mut decimal_part = 1;
    while f > f as i32 as f32 {
        decimal_part *= 10;
        f *= 10.0;
    }

    let f: i32 = f as i32;

    s = add_integer(s, f / decimal_part, 0);
    s.push('.');
    s = add_integer(s, f % decimal_part, 0);

    s
}

fn ex3() {
    let mut s: String = String::new();
    s = add_space(s, 22);
    s = add_str(s, "I 💚".to_string());
    println!("{s}");
    s = "".to_string();
    s = add_space(s, 22);
    s = add_str(s, "RUST.".to_string());
    println!("{s}");

    s = "".to_string();
    s = add_str(s, "Most".to_string());
    s = add_space(s, 12);
    s = add_str(s, "crate".to_string());
    s = add_space(s, 5);
    s = add_integer(s, 306_437_968, 0);
    s = add_space(s, 11);
    s = add_str(s, "and".to_string());
    s = add_space(s, 5);
    s = add_str(s, "latest".to_string());
    s = add_space(s, 9);
    s = add_str(s, "is".to_string());
    println!("{s}");

    s = "".to_string();
    s = add_space(s, 5);
    s = add_str(s, "downloaded".to_string());
    s = add_space(s, 7);
    s = add_str(s, "has".to_string());
    s = add_space(s, 13);
    s = add_str(s, "downloads".to_string());
    s = add_space(s, 5);
    s = add_str(s, "the".to_string());
    s = add_space(s, 8);
    s = add_str(s, "version".to_string());
    s = add_space(s, 4);
    s = add_float(s, 2.038);
    println!("{s}");
}

fn main() {
    ex1();
    ex2();
    ex3();
}
