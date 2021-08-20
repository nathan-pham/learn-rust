use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100); // 1..100 (exclusive), 1..=100 (inclusive)

    println!("{}", secret);

    loop {
        println!("Enter your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue
        };
        
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    };


    // println!("You guessed: {}", guess);
    // println!("The secret number is: {}", secret);


}