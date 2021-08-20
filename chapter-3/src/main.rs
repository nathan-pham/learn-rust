// constant, always immutable variable
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("MAX_POINTS: {}", MAX_POINTS);

    // mutable variable
    let mut x = 5;
    println!("Initial value of x: {}", x);

    x = 2;
    println!("Reassigned value of x: {}", x);

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y + 10;
    println!("Final value of y: {}", y);
}