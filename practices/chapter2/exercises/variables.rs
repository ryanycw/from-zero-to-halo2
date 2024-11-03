use std::io;

fn main() {
    // Exercise 1: Understanding Mutability
    println!("\n=== Exercise 1: Mutability ===");
    // Error case
    let x = 5;
    println!("Original x: {}", x);
    // x = 6;  // This would cause a compile error

    // Solution: Make the variable mutable
    let mut y = 5;
    println!("Original y: {}", y);
    y = 6;
    println!("Modified y: {}", y);

    // Exercise 2: Different ways to declare mutable variables
    println!("\n=== Exercise 2: Variable Declaration ===");
    // Simple numeric variable
    let mut count = 0;
    count += 1;
    println!("Count: {}", count);

    // String variable
    let mut message = String::from("Hello");
    message.push_str(", World!");
    println!("Message: {}", message);

    // Using type annotations
    let mut counter: u32 = 32;
    println!("Before: Counter: {}", counter);
    counter = 42;
    println!("After: Counter: {}", counter);

    // Exercise 3: Type annotations and conversions
    println!("\n=== Exercise 3: Type Annotations ===");
    let integer: i32 = -42;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀';

    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);

    // Type conversion example
    let float_to_int: i32 = float as i32;
    println!("Float converted to integer: {}", float_to_int);

    // Exercise 4: Working with numeric bounds
    println!("\n=== Exercise 4: Numeric Bounds ===");
    // Maximum values
    let max_u8: u8 = u8::MAX;
    let max_i8: i8 = i8::MAX;
    println!("Maximum u8: {}", max_u8);
    println!("Maximum i8: {}", max_i8);

    // This would cause overflow in debug mode
    // let overflow: u8 = 256;  // Compile error

    // Safe handling of potential overflow
    let big_number = 1000;
    let converted = match u8::try_from(big_number) {
        Ok(num) => num,
        Err(_) => {
            println!("Could not convert {} to u8, using max value", big_number);
            u8::MAX
        }
    };
    println!("Safely handled conversion: {}", converted);

    // Bonus: Interactive example
    println!("\n=== Bonus: Interactive Example ===");
    println!("Enter a number between 0 and 255:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse input and handle potential errors
    match input.trim().parse::<u8>() {
        Ok(num) => println!("Successfully parsed u8: {}", num),
        Err(e) => println!("Error parsing number: {}. Using 0 instead.", e),
    }
}
