
fn main() {
    // this is a string literal which is immutable
    let s = "hello";

    // this is a String instance which is mutable
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // string literals can be copied this way because their size is known
    let s = "test";
    let a = s;
    println!("s: {}, a: {}", s, a);

    // this would not work
    // let s = String::from("hello");
    // let a = s;
    // here the code breaks because we are trying to access s which is invalidated and moved to a
    // println!("s: {}, a: {}", s, a);
}
