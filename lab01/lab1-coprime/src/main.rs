fn is_coprime(nr1: i32, nr2: i32) -> bool {
    let mut i: i32 = 2;
    let max_interval: i32 = if nr1 < nr2 { nr1 } else { nr2 };
    while i <= max_interval {
        if nr1 % i == nr2 % i && nr2 % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
fn main() {
    for i in 1..100 {
        for j in 1..100 {
            println!("{} - {} : {}", i, j, is_coprime(i, j));
        }
    }
}
