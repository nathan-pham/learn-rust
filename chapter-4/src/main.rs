fn main() {
    let s = String::from("Hello World");

    // invalidates s, moves pointer to s1
    let (s1, length) = calculate_length(s);
    println!("string: {}, length: {}", s1, length);

    // shadow previous s
    let s = String::from("Hello World");
    let length = calculate_length_references(&s);
    println!("string: {}, length: {}", s, length);
}

fn calculate_length(s: String) -> (String, usize) {
    // simply returning (s, s.len()) would not work because s would be invalidated

    let length = s.len();
    (s, length)
}

// refers to value of s but does not own it
// simply borrows the value of "s", it cannot mutate it (say, by using s.push_str("additional string"))
fn calculate_length_references(s: &String) -> usize {
    s.len()
}