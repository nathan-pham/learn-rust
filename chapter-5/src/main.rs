// declare a User struct
struct User {
    name: String,
    email: String,
    active: bool
}

// declare tuple structs
struct Point(i32, i32, i32);

fn main() {
    let user = create_user(String::from("Nathan Pham"), String::from("nathanpham.me@gmail.com"));
    print_user(user);

    // "spread" syntax, only applies to attributes that can be copied
    let extended = User { name: String::from("Bruh"), email: String::from("bruh@gmail.com"), ..user };
    print_user(extended);

    let point = Point(0, 0, 0);
}

// create instance of a struct
fn create_user(name: String, email: String) -> User {
    User { name, email, active: false }
}

// print user
fn print_user(user: User) {
    println!("name: {} \nemail: {} \nactive: {}", user.name, user.email, user.active);
}