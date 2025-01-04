#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let width = 10;
    let height = 20;

    let rect1: (u32, u32) = (10, 20);
    let mut rect2 = Rectangle { height: 10, width: 20 };
    let area = area_rect(&rect2);

    println!("Area of rectangle is {area}");
    println!("Rectangle is {rect2:?}");
    dbg!(&rect2);
    rect2.height = 30;
    println!("Rectangle is {rect2:?}");
}

fn area_of_rectangle(w: u32, h: u32) -> u32 {
    w * h
}

fn area_of_rect(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area_rect(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
