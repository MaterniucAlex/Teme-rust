fn main() {
    for i in (1..100).rev() {
        if i == 1 {
            println!(
                "
{} bottle of beer on the wall,
{} bottle of beer.
Take one down, pass it around,
No bottles of beer on the wall.",
                i, i
            );
        } else {
            println!(
                "
{} bottles of beer on the wall,
{} bottles of beer.
Take one down, pass it around,
{} bottles of beer on the wall.",
                i,
                i,
                i - 1
            );
        }
    }
}
