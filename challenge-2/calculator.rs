use std::collections::HashMap;
use std::io;

#[derive(Clone)]
enum Operation {
    Add { a: f64, b: f64 },
    Subtract { a: f64, b: f64 },
    Multiply { a: f64, b: f64 },
    Divide { a: f64, b: f64 },
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add { a, b } => a + b,
        Operation::Subtract { a, b } => a - b,
        Operation::Multiply { a, b } => a * b,
        Operation::Divide { a, b } => a / b,
    }
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut operation = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for first number.");
            return;
        }
    };

    println!("Enter second number:");
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: f64 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for second number.");
            return;
        }
    };

    println!("Enter the operation:");
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let operation = operation.trim();

    let operations = HashMap::from([
        ("+", Operation::Add { a, b }),
        ("-", Operation::Subtract { a, b }),
        ("*", Operation::Multiply { a, b }),
        ("/", Operation::Divide { a, b }),
    ]);

    let result = match operations.get(&operation) {
        Some(op) => calculate(op.clone()),
        None => {
            println!("Invalid operation.");
            return;
        }
    };

    println!("Result: {}", result);
}
