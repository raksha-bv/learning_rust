fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
}
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
