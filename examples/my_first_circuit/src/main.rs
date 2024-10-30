fn get_greeting(name: &str) -> String {
    format!("Welcome to Rust, {}!", name)
}

fn main() {
    let greeting = get_greeting("Halo2 Developer");
    println!("{}", greeting);
}
