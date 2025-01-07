fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let mut v: Vec<i32> = Vec::new();
    v = vec![1, 2, 3];
    println!("{:?}", v);
    v.push(4);
    println!("{:?}", v);

    let third: &i32 = &v[2];

    println!("The third element is {third}");
    v.push(5);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
    for cell in row.iter() {
        match cell {
            SpreadsheetCell::Int(n) => println!("Int: {n}"),
            SpreadsheetCell::Float(f) => println!("Float: {f}"),
            SpreadsheetCell::Text(t) => println!("Text: {t}"),
        }
    }
}
