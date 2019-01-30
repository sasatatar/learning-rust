fn main() {
    println!("Hello, world!");

    greet("Peter");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}