fn main() {

let mut s = String::from("HelloWorld");
// let len = first_word(&s);

// s.clear();
// println!(" {s} -> {len}");
let s = String::from("Hello world");

let hello = &s[0..5];
let world = &s[6..11];

println!(" {hello} {world}");

let str_ = first_word(&s);
// s.clear(); // error!
println!(" {str_}");

// String Literals as Slices
let s = "Hello, world!";
let hello = &s[0..6];
let world = &s[7..13];

println!(" {hello} {world}");

}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
let bytes = s.as_bytes();
for (i, &item) in bytes.iter().enumerate() {
if item == b' ' {
return &s[0..i];
}
}
&s[..]
}