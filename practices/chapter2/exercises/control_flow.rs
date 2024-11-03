use std::io::{self, Write};

// Helper function to get user input
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    // Exercise 1 Solution: FizzBuzz
    for i in 1..=100 {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }

    // Exercise 2 Solution: Fibonacci
    let mut a = 0;
    let mut b = 1;
    for _ in 0..10 {
        println!("{}", a);
        let temp = a + b;
        a = b;
        b = temp;
    }

    // Exercise 3 Solution: Countdown
    let mut count = 10;
    while count > 0 {
        println!("{}", count);
        if count == 5 {
            println!("Halfway there!");
        }
        count -= 1;
    }

    // Exercise 4 Solution: Calculator
    fn calculator(op: char, a: f64, b: f64) -> Option<f64> {
        match op {
            '+' => Some(a + b),
            '-' => Some(a - b),
            '*' => Some(a * b),
            '/' => {
                if b != 0.0 {
                    Some(a / b)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    loop {
        println!("\nSimple Calculator");
        println!("Operations: +, -, *, / (or 'q' to quit)");

        let operation = get_input("Enter operation: ");
        if operation == "q" {
            println!("Goodbye!");
            break;
        }

        let op = operation.chars().next().unwrap_or(' ');
        if !['+', '-', '*', '/'].contains(&op) {
            println!("Invalid operation!");
            continue;
        }

        // Get first number
        let num1_str = get_input("Enter first number: ");
        let num1: f64 = match num1_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

        // Get second number
        let num2_str = get_input("Enter second number: ");
        let num2: f64 = match num2_str.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            }
        };

        // Perform calculation
        match calculator(op, num1, num2) {
            Some(result) => println!("{} {} {} = {}", num1, op, num2, result),
            None => println!("Error: Division by zero or invalid operation"),
        }
    }
}
