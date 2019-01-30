use std::io;

fn main() {
    println!("Hello, world!");

    println!("What is your name?");

    let mut name = String::new();

    io::stdin().read_line(&mut name)
            .expect("Failed to read line");

    greet(&name.trim());

    println!("Can we still use name: {}", name);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}