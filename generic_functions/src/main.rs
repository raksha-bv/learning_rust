struct Point<T, U> {
    x: T,
    y: U,
}
fn find_largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let num_list = vec![1, 44, 64, 33, 4, 44];
    let result = find_largest(&num_list);
    println!("Result is {}", result);
    let num_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = find_largest(&num_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = find_largest(&char_list);
    println!("The largest char is {result}");
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
