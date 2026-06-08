

The “Storing Values with Variables” section, by default, variables are
immutable. This is one of many nudges Rust gives you to write your code in a way that takes
advantage of the safety and easy concurrency that Rust offers. However, you still have the
option to make your variables mutable.

Data Type

On the right of the equal sign is the value that guess is bound to, which is the
result of calling String::new , a function that returns a new instance of a String . String is a
string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

let apples = 5; // immutable
let mut bananas = 5; // mutable