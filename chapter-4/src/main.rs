fn main() {
    let s = String::from("Hello World");

    // invalidates s, moves pointer to s1
    let (s1, length) = calculate_length(s);
    println!("string: {}, length: {}", s1, length);
}

fn calculate_length(s: String) -> (String, usize) {
    // simply returning (s, s.len()) would not work because s would be invalidated

    let length = s.len();
    (s, length)
}