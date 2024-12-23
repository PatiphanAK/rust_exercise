use std::io;

fn quadeq(a: i32, b: i32, c: i32) -> (f64, f64) {
    let d = (b as f64).powi(2) - 4.0 * a as f64 * c as f64;
    if d < 0.0 {
        return (0.0, 0.0);
    }
    let x1 = (-b as f64 + d.sqrt()) / (2.0 * a as f64);
    let x2 = (-b as f64 - d.sqrt()) / (2.0 * a as f64);
    (x1, x2)
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    
    println!("This is exercise 2");
    
    println!("Enter a: ");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: i32 = a.trim().parse().expect("Please enter a valid integer for a");
    
    println!("Enter b: ");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: i32 = b.trim().parse().expect("Please enter a valid integer for b");

    println!("Enter c: ");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: i32 = c.trim().parse().expect("Please enter a valid integer for c");

    let (root1, root2) = quadeq(a, b, c);
    println!("The expression is {}x^2 + {}x + {}", a, b, c);
    if root1 == 0.0 && root2 == 0.0 {
        println!("No real roots");
    } else {
        println!("Root 1: {}", root1);
        println!("Root 2: {}", root2);
    }
}
