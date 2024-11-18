use std::io;


fn main() {
    // Prompt the user for input
    println!("Enter the first number:");
    let mut num1: String = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");

    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim();

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

    // Determine the operation and calculate the result
    let result = match operation {
        "+" => calculate(Operation::Add(num1, num2)),
        "-" => calculate(Operation::Subtract(num1, num2)),
        "*" => calculate(Operation::Multiply(num1, num2)),
        "/" => calculate(Operation::Divide(num1, num2)),
        _ => {
            println!("Error: Invalid operation");
            return;
        }
    };

    match result {
        Some(value) => println!("Result of ({num1} {operation} {num2}) is: {value}"),
        None => println!("Divison by Zero is not allowed!")
    }
}


enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64)
}

fn calculate(operation: Operation) -> Option<f64> {
    match operation {
        Operation::Add(a, b) => {
            Some(a + b)
        },
        Operation::Subtract(a, b) => {
            Some(a - b)
        },  
        Operation::Multiply(a, b) => {
            Some(a * b)
        },
        Operation::Divide(a, b) => {
            if b != 0.0 {
                Some(a/b)
            } else {
                None
            }
        }
    }
}