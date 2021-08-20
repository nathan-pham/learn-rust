// constant, always immutable variable
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("MAX_POINTS: {}", MAX_POINTS);

    // mutable variable
    let mut x = 5;
    println!("Initial value of x: {}", x);

    x = 2;
    println!("Reassigned value of x: {}", x);

    // shadowing (allows for type changes)
    let y = 5;
    let y = y + 1;
    let y = y + 10;
    println!("Final value of y: {}", y);

    let r_string = "Random String";
    let r_string = r_string.len();
    println!("Final value of r_string: {}", r_string);

    // must define type in order to parse strings
    let guess: f32 = "42.1".parse().expect("This ain't it chief.");
    println!("Guess: {}", guess);

    // tuples
    let tup: (i32, f64) = (1, 5.3);
    println!("Tuple: {:?}", tup);

    let (a, b) = tup;
    println!("tup.0: {}, tup.1: {}", a, b);

    // arrays (defined length)
    // i32 is the type for 3 elements
    let arr: [i32; 3] = [1, 2, 3];
    println!("Array: {:?}", arr);

    // array with repeated values (10)
    let arr = [3; 10];
    println!("Array: {:?}", arr);

    println!("Array[0]: {}", arr[0]);
}