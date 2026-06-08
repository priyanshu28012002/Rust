use std::io;
fn main() {
    // let muteable
    let mut x = 12;
    println!("{x}");
    x = 23;
    println!("{x}");
    // const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");
    // shadowing
    let mut apple = 10;
    apple = 23;
    let apple = 10;
    let apple = 23;
    let apple = 10;
    println!("{apple}");
    let apple = true;
    println!("{apple}");
    {
        let apple = 32;
        println!("{apple}");
    }
    println!("{apple}");
    let mut age = 24;
    age = age + 10;
    let age = age;
    println!("{age}");

    // Data Type

    // Tuple Type
    // No add and remove

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {five_hundred}");

    println!("The value of y is: {y}");
    // Unit

    let tup: () = ();
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [3; 5];

    let first = a[0];
    let second = a[1];

let a = [1, 2, 3, 4, 5];
println!("Please enter an array index.");
let mut index = String::new();
io::stdin()
.read_line(&mut index)
.expect("Failed to read line");

let index: usize = 0
.trim()
.parse()
.expect("Index entered was not a number");

let element = a[index];
println!("The value of the element at index {index} is: {element}");
}
