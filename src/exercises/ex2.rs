use std::io;

fn quadeq(a: i32, b: i32, c: i32) -> Option<(f64, f64)> {
    let discriminant = (b as f64).powi(2) - 4.0 * a as f64 * c as f64;
    if discriminant < 0.0 {
        None
    } else {
        let x1 = (-b as f64 + discriminant.sqrt()) / (2.0 * a as f64);
        let x2 = (-b as f64 - discriminant.sqrt()) / (2.0 * a as f64);
        Some((x1, x2))
    }
}

fn read_input(prompt: &str) -> i32 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Please enter a valid integer")
}

fn display_roots(a: i32, b: i32, c: i32, roots: Option<(f64, f64)>) {
    println!("The expression is {}x^2 + {}x + {}", a, b, c);
    match roots {
        Some((root1, root2)) => {
            println!("Root 1: {}", root1);
            println!("Root 2: {}", root2);
        }
        None => println!("No real roots"),
    }
}

fn main() {
    let a = read_input("Enter a:");
    let b = read_input("Enter b:");
    let c = read_input("Enter c:");

    let roots = quadeq(a, b, c);
    display_roots(a, b, c, roots);
}
