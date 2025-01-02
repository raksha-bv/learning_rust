fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    print_labeled_measurement(5, 'h');

    let x = five();

    println!("The value of x is: {x}");

    let z = plus_one(5);

    println!("The value of z is: {z}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
