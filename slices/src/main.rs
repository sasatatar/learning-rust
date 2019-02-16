fn main() {
    let s = String::from("Petar");

    let s2 = "test";

    let len = first_word(&s);

    println!("len: {}", len);
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}