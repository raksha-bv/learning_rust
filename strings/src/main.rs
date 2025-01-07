fn main() {
    let mut s = String::new();
    s = "hello".to_string();
    println!("{}", s);
    s.push_str(" world");
    println!("{}", s);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s4 = format!("{s1}-{s2}-{s3}");
    println!("{}", s4);
    println!("{}", s1);

    for c in "Здравствуйте".chars() {
        println!("{}", c);
    }
}
