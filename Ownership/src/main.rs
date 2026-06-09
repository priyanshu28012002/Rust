fn main() {
    let str_ = "kja";
    let mut str_ = String::from("Hello ");
    str_.push_str("World");

    {
        str_.push_str(" Kalu");

        println!("{}", str_);
    }
    // In this case the Expected output is Hello World
    println!("{}", str_);

    //Variables and Data Interacting with Move
    let mut x = 5;
    let y = x; // copy

    x = 19;

    println!("X is {x} Y is {y}");

    let s1 = String::from("hello -> ");
    let s2 = s1.clone(); // Ownwe ship transfer
    // s1 value borrowed here after move

    println!("s1 is {s1} s2 is {s2}");

    // String have 3 part
    // ptr -> heap index, value 0,h 1,e 2,l 3,l 4,o
    // len
    // capcity

    // let y = y;

    // s1
    // ptr -> heap index, value 0,h 1,e 2,l 3,l 4,o
    // len
    // capcity

    // s2
    // ptr -> heap index, value 0,h 1,e 2,l 3,l 4,o
    // len
    // capcity

    // Variables and Data Interacting with Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    //Stack-Only Data: Copy
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    // Ownership and Functions

    let num = 5;
    let num2 = add(num);
    makes_copy(num);

    println!("num = {num}, num2 = {num2}");

    let name = String::from("Priyanshu");
    takes_ownership(name.clone());

    println!("name = {name}");

    // Return ownership. Values and Scope

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    // gives_ownership moves its return
    // value into s1
    // s2 comes into scope

    let s3 = takes_and_gives_back(s2.clone()); // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3

    println!("s1 = {s1}, s2 = {s2} s3 = {s3}");
    // we want to let a function use a value but not take ownership
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1.clone());
    println!("The length of '{s2}' is {len}.");

    let mut s1 = String::from("Hi");

    // by Default Referance is Imutable
    let len = calculate_length_(&s1); // 

    // Mutable References
    change(&mut s1);

    let mut s = String::from("hello");

    // multipal muabable referance

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{r1}, {r2}");

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    // Dangling References
}

fn add(x: i32) -> i32 {
    x + 10
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!(" takes_ownership {some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string
    // some_string is returned and
    // moves out to the calling
    // function
}
// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope
    a_string
    // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// Referance and Borrowing

fn calculate_length_(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Dangalling Ref

fn dangle() -> String {
    let s = String::from("Missing");
    s
}