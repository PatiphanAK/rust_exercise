use std::io;

fn assign_value() -> String {
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read input");
    value.trim().to_string() // ลบช่องว่างและคืนค่าเป็น String
}

fn assign_number() -> i32 {
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read input");
    value.trim().parse().expect("Please enter a valid number") // แปลงเป็น i32
}

struct User {
    name: String,
    age: i32,
}

fn main() {
    println!("Enter your name: ");
    let name = assign_value();

    println!("Enter your age: ");
    let age = assign_number();

    let user = User { name, age };

    println!("User created: Name - {}, Age - {}", user.name, user.age);
}
