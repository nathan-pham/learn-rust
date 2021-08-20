fn main() {
    print_max_points();
    print_mutable_var();
    print_shadowing();
    print_guess();
    print_tuples_arrays();
    print_parameters(100, "example_string");

    let original_n = 10;
    println!("original_n: {}, add one: {}", original_n, add_one(10));

    print_if_else();
    print_counters();
}

// constant, always immutable variable
const MAX_POINTS: u32 = 100_000;
fn print_max_points() {
    println!("MAX_POINTS: {}", MAX_POINTS);
}

// mutable variable
fn print_mutable_var() {
    let mut x = 5;
    println!("Initial value of x: {}", x);

    x = 2;
    println!("Reassigned value of x: {}", x);

}

// shadowing (allows for type changes)
fn print_shadowing() {
    let y = 5;
    let y = y + 1;
    let y = y + 10;
    println!("Final value of y: {}", y);

    let r_string = "Random String";
    let r_string = r_string.len();
    println!("Final value of r_string: {}", r_string);
}

fn print_guess() {
    // must define type in order to parse strings
    let guess: f32 = "42.1".parse().expect("This ain't it chief.");
    println!("Guess: {}", guess);
}

fn print_tuples_arrays() {
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

// example function with parameters
fn print_parameters(a: i32, b: &str) {
    println!("a: {}, b: {}", a, b);
}

// example function with retun
fn add_one(n: i32) -> i32 {
    n + 1
}

// control flow with if, else
fn print_if_else() {
    let number = 4;

    if number < 3 {
        println!("number < 3");
    } else {
        println!("number >= 3");
    }

    // "ternary operator"
    let number = if number == 4 { "four" } else { "five" };
    println!("number: {}", number);
}

// counters & loops
fn print_counters() {
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };
    println!("counter result: {}", result);

    // while loop
    let mut liftoff = 3;
    while liftoff > 0 {
        println!("{}", liftoff);
        liftoff -= 1;
    };
    println!("Lift off!!!");

    // for loop
    let array: [i32; 4] = [10, 11, 12, 13];
    for element in array.iter() {
        println!("{}", element);
    }
    for i in (0..10).rev() {
        println!("{}", i);
    }
}