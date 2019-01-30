fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // s1 is moved and is not available any more

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}