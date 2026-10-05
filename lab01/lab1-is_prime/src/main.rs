fn is_prime(nr: i32) -> bool {
    let mut i: i32 = 2;
    while i <= nr / 2 {
        if nr % i == 0 {
            return false;
        }
        i = i + 1 + i % 2;
    }
    true
}
fn main() {
    for i in 1..100 {
        println!("{} - {}", i, is_prime(i));
    }
}
