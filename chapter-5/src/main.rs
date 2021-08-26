// declare a User struct
struct User {
    name: String,
    email: String,
    active: bool
}

// declare tuple struct
#[derive(Debug)]
struct Point(i32, i32, i32);

// declare a rectangle struct
// enables println! with debugging information
#[derive(Debug)]
struct Rectangle { 
    width: u32, 
    height: u32 
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.area() > rectangle.area()
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let user = create_user(String::from("Nathan Pham"), String::from("nathanpham.me@gmail.com"));

    // "spread" syntax, only applies to attributes that can be copied
    let extended = User { name: String::from("Bruh"), email: String::from("bruh@gmail.com"), ..user };
    print_user(user);
    print_user(extended);

    let point = Point(0, 0, 0); 
    println!("point: {:?}", point);

    let rectangle = Rectangle { width: 100, height: 100 };
    let rectangle2 = Rectangle { width: 150, height: 150 };
    println!("rectangle: {:?}", rectangle);
    println!("rectangle area 1: {}", rectangle.area());
    println!("rectangle area 2: {}", rectangle2.area());
    println!("can rectangle 1 hold rectangle 2? {}", rectangle.can_hold(&rectangle2));

    println!("rectangle 3: {:?}", Rectangle::square(3));
}

// create instance of a struct
fn create_user(name: String, email: String) -> User {
    User { name, email, active: false }
}

// print user
fn print_user(user: User) {
    println!("name: {} \nemail: {} \nactive: {}", user.name, user.email, user.active);
}