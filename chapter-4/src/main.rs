fn main() {
    let s = String::from("Hello World");

    // invalidates s, moves pointer to s1
    let (s1, length) = calculate_length(s);
    println!("string: {}, length: {}", s1, length);

    // shadow previous s
    let s = String::from("Hello World");
    let length = calculate_length_references(&s);
    println!("string: {}, length: {}", s, length);

    // shadow previous s (again)
    let s = String::from("Hello");
    let slice = get_slices(&s);
    println!("string slice: {}", slice)
}

fn calculate_length(s: String) -> (String, usize) {
    // simply returning (s, s.len()) would not work because s would be invalidated

    let length = s.len();
    (s, length)
}

// refers to & borrows the value of s but does not own it
// cannot mutate it (ie:  s.push_str("additional string"))
// you can create one mutatable references (&mut s) perscope
fn calculate_length_references(s: &String) -> usize {
    s.len()
}

// rust prevents dangling references (&) that are never used outside of a scope
// returning s would be fine because it also returns ownership
// fn dangler() -> &String {
//     let s = String::from("Hello World");
//     &s
// }

// string slices: references part of a String
fn get_slices(s: &String) -> &str {
    &s[0..1]
}