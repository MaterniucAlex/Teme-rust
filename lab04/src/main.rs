fn p1() {
    let content = std::fs::read_to_string("text.txt");
    let mut content = content.unwrap_or("".to_string());
    let mut end_row: usize = content.find('\n').unwrap_or(0);

    let mut max_bytes = 0;
    let mut max_bytes_row = "".to_string();
    let mut max_chars = 0;
    let mut max_chars_row = "".to_string();

    while end_row > 0 {
        let row = &content[..end_row].to_string();
        let bytes = row.len();
        let mut chars: usize = 0;
        for _ in row.chars() {
            chars += 1;
        }
        content = content[end_row + 1..].to_string();
        end_row = content.find('\n').unwrap_or(0);

        if max_bytes < bytes {
            max_bytes = bytes;
            max_bytes_row = row.to_string();
        }

        if max_chars < chars {
            max_chars = chars;
            max_chars_row = row.to_string();
        }
    }

    println!("Bytes: {}, {}", max_bytes_row, max_bytes);
    println!("Chars: {}, {}", max_chars_row, max_chars);
}

fn p2() {
    let original_string: String = String::from("I wanna kms soo bad today");
    let mut copy_string: String = String::new();
    for my_char in original_string.chars() {
        if !my_char.is_ascii() {
            println!("Character {my_char} is not ascii");
            return;
        }
        if !my_char.is_alphabetic() {
            copy_string.push(my_char);
            continue;
        }
        let left_padding: u8 = if my_char.is_lowercase() { 97 } else { 65 };

        let nr_of_letters = 26;
        let ammount_to_shift = 13;
        let new_char =
            (my_char as u8 - left_padding + ammount_to_shift) % nr_of_letters + left_padding;
        let new_char = new_char as char;
        copy_string.push(new_char);
    }
    println!("{copy_string}");
}

fn p3() {
    let content = std::fs::read_to_string("text3.txt");
    let mut content = content.unwrap_or("".to_string());

    for abreviation in ["pt", "ptr", "dl", "dna"] {
        while let Some(index) = content.find(abreviation) {
            let index: usize = index;
            let pre = content[..index].to_string();
            let post = &content[index + abreviation.len()..].to_string();
            content = pre;
            if abreviation == "pt" || abreviation == "ptr" {
                content.push_str("pentru");
            }
            if abreviation == "dl" {
                content.push_str("domnul");
            }
            if abreviation == "dna" {
                content.push_str("doamna");
            }
            content.push_str(post);
        }
    }

    print!("{content}");
}

fn p4() {
    let content = std::fs::read_to_string("/etc/hosts");
    let mut content = content.unwrap_or("".to_string());
    while let Some(end_row) = content.find('\n') {
        let row = content[..end_row].to_string();
        if row.starts_with('#') {
            content = content[end_row + 1..].to_string();
            continue;
        }

        let first: String = if let Some(end_first_column) = row.find(' ') {
            row[..end_first_column].to_string()
        } else {
            "".to_string()
        };

        let last: String = if let Some(stripped) = row.strip_prefix(first.as_str()) {
            stripped.trim_start().to_string()
        } else {
            "".to_string()
        };

        let mut line = last;
        line.push_str(" => ");
        line.push_str(first.as_str());

        println!("{}", line);
        content = content[end_row + 1..].to_string();
    }
}

fn main() {
    println!("1)");
    p1();
    println!("2)");
    p2();
    println!("3)");
    p3();
    println!("4)");
    p4();
}
