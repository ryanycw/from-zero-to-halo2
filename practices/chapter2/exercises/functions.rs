// Exercise 1 Solution
fn string_length(s: &str) -> usize {
    s.chars().count() // Handles Unicode correctly
}

// Exercise 2 Solution
fn find_min_max(numbers: &[i32]) -> (i32, i32) {
    let mut min = numbers[0];
    let mut max = numbers[0];

    for &num in numbers {
        if num < min {
            min = num;
        }
        if num > max {
            max = num;
        }
    }

    (min, max)
}

// Exercise 3 Solution
fn fahrenheit_to_all(f: f64) -> (f64, f64) {
    let celsius = (f - 32.0) * 5.0 / 9.0;
    let kelvin = celsius + 273.15;
    (celsius, kelvin)
}

// Exercise 4 Solution
fn is_prime(n: i32) -> Result<bool, String> {
    if n < 0 {
        return Err("Negative numbers cannot be prime".to_string());
    }
    if n <= 1 {
        return Ok(false);
    }

    for i in 2..=(n as f64).sqrt() as i32 {
        if n % i == 0 {
            return Ok(false);
        }
    }
    Ok(true)
}

fn main() {
    // Test Exercise 1
    assert_eq!(string_length("hello"), 5);
    assert_eq!(string_length("你好"), 2);

    // Test Exercise 2
    let numbers = [1, 5, 3, 8, 2];
    let (min, max) = find_min_max(&numbers);
    assert_eq!(min, 1);
    assert_eq!(max, 8);

    // Test Exercise 3
    let (c, k) = fahrenheit_to_all(32.0);
    assert_eq!(c, 0.0);
    assert_eq!(k, 273.15);

    // Test Exercise 4
    assert_eq!(is_prime(17), Ok(true));
    assert_eq!(is_prime(4), Ok(false));
    assert!(is_prime(-5).is_err());
}
