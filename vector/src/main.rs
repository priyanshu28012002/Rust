fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(4);
    vec.push(3);
    vec.push(2);
    println!("{vec:?}");

    let mut vec = Vec::new();
    vec.push("d");
    vec.push("d");
    vec.push("2");
    vec.push("ds");

    println!("{vec:?}");

    let v: Vec<i32> = Vec::new(); // empty
    let mut v = vec![1, 3, 4]; // initial

    // Reading Elements of Vectors

    let v = vec![1, 2, 3, 4, 5];
    let forth = v[4];
    let third: &i32 = &v[2];

    println!("The forth element is {forth}");

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; error
    // v.push(6);
    println!("The first element is: {}", v[0]);

    // Iterating Over the Values in a Vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
    // Using an Enum to Store Multiple Types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Float(10.12),
    ];
    println!("The first element is: {:?}", row);


    // match row.get(1){
    //     Some(SpreadsheetCell::Int(value:&i32))=> println!("This value is {value}");
    //     _ =>();
    // }
    // Dropping a Vector Drops Its Elements

    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
