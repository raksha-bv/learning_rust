fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&mut s1);
    let num = 5;
    function(num);
    println!("The number is {num}");

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(" world");
    s.len()
}

fn function(n: u32) {
    println!("The number is {n}");
}
