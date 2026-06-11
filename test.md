# Chapter 11 — Writing Automated Tests (Implementation-Focused Notes)

Chapter 11 teaches **how Rust testing works internally and how to implement tests in real projects**. 

---

# 1. Basic Test Structure

When Cargo creates a library project, it generates:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

Key points:

```rust
#[cfg(test)]
```

Compile this module only during:

```bash
cargo test
```

Not during:

```bash
cargo build
```

This keeps production binaries smaller. 

---

## `#[test]`

Marks a function as a test.

```rust
#[test]
fn my_test() {
}
```

The Rust test runner automatically discovers and executes it. 

---

# 2. Running Tests

```bash
cargo test
```

Example output:

```text
running 1 test
test tests::it_works ... ok

test result: ok.
1 passed
0 failed
```



---

# 3. Creating a Failing Test

```rust
#[test]
fn another() {
    panic!("Make this test fail");
}
```

Output:

```text
test tests::another ... FAILED
```

A test fails whenever it panics. 

---

# 4. `assert!`

Used when checking a boolean condition.

Implementation example:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width
            && self.height > other.height
    }
}
```

Test:

```rust
#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };

    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}
```

If condition is false:

```rust
assert!(false);
```

the test fails. 

---

## Negative Assertion

```rust
#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };

    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(!smaller.can_hold(&larger));
}
```

---

# 5. `assert_eq!`

Checks equality.

```rust
#[test]
fn addition() {
    assert_eq!(2 + 2, 4);
}
```

Equivalent logic:

```rust
if left != right {
    panic!();
}
```

Used heavily in Rust tests. 

---

# 6. `assert_ne!`

Checks values are NOT equal.

```rust
#[test]
fn not_equal() {
    assert_ne!(5, 10);
}
```

Passes because values differ.

---

# 7. Custom Failure Messages

```rust
#[test]
fn custom_message() {
    let value = 5;

    assert!(
        value > 10,
        "value should be > 10 but got {}",
        value
    );
}
```

Output:

```text
value should be > 10 but got 5
```

Makes debugging easier.

---

# 8. Testing Panic Conditions

Sometimes success means:

> "The code must panic."

Example:

```rust
fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("division by zero");
    }

    a / b
}
```

Test:

```rust
#[test]
#[should_panic]
fn divide_by_zero() {
    divide(10, 0);
}
```

Test passes only if panic occurs.

---

## Panic Message Matching

```rust
#[test]
#[should_panic(expected = "division by zero")]
fn divide_by_zero() {
    divide(10, 0);
}
```

Now panic message must contain:

```text
division by zero
```

otherwise test fails.

---

# 9. Returning `Result` From Tests

Instead of panicking:

```rust
#[test]
fn test_file() -> Result<(), String> {

    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("math broken"))
    }
}
```

Advantages:

* cleaner error handling
* useful when calling functions returning `Result`

---

# 10. Controlling Test Execution

---

## Run All Tests

```bash
cargo test
```

---

## Run Single Test

```bash
cargo test exploration
```

Runs only tests matching:

```rust
fn exploration()
```



---

## Run Tests Matching Pattern

```bash
cargo test rectangle
```

Runs all tests containing:

```text
rectangle
```

in their names.

---

# 11. Showing `println!` Output

By default:

```rust
println!("hello");
```

is hidden.

Show output:

```bash
cargo test -- --show-output
```

or

```bash
cargo test -- --nocapture
```

---

# 12. Running Tests Sequentially

Normally Rust runs tests in parallel.

Force single-thread execution:

```bash
cargo test -- --test-threads=1
```

Useful when:

* files
* database
* global state

are shared.

---

# 13. Ignoring Expensive Tests

```rust
#[test]
#[ignore]
fn huge_database_test() {
}
```

Normal run:

```bash
cargo test
```

Ignored.

Run ignored tests:

```bash
cargo test -- --ignored
```

---

# 14. Unit Tests

Unit tests live in same file as implementation.

Example:

```rust
src/lib.rs
```

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2,3), 5);
    }
}
```

---

## Why `use super::*`

```rust
mod tests {
    use super::*;
}
```

`super` means:

```text
parent module
```

Allows tests to access code being tested. 

---

# 15. Testing Private Functions

Rust allows testing private functions.

```rust
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
```

Test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(internal_adder(2,3), 5);
    }
}
```

This is legal because child modules can access parent items. 

---

# 16. Integration Tests

Stored separately:

```text
project/
│
├── src/
│   └── lib.rs
│
└── tests/
    └── integration_test.rs
```

Cargo treats every file inside:

```text
tests/
```

as a separate crate. 

Example:

```rust
use adder::add;

#[test]
fn it_adds_two() {
    assert_eq!(add(2,2), 4);
}
```

Run:

```bash
cargo test
```

or

```bash
cargo test --test integration_test
```



---

# 17. Shared Setup for Integration Tests

Wrong:

```text
tests/
├── common.rs
├── integration_test.rs
```

Cargo treats:

```text
common.rs
```

as a test crate and runs it. 

Correct:

```text
tests/
├── common/
│   └── mod.rs
└── integration_test.rs
```



Example:

```rust
// tests/common/mod.rs

pub fn setup() {
}
```

```rust
mod common;

#[test]
fn test_one() {
    common::setup();
}
```



---

# 18. Binary Crate Limitation

If project only contains:

```text
src/main.rs
```

you cannot directly write integration tests against internal functions.

Best practice:

```text
src/
├── lib.rs
└── main.rs
```

Move logic into:

```rust
lib.rs
```

Test there.

Keep `main.rs` thin. 

---

# Chapter 11 Cheat Sheet

| Feature                  | Syntax                           |
| ------------------------ | -------------------------------- |
| Test Function            | `#[test]`                        |
| Force Failure            | `panic!()`                       |
| Boolean Check            | `assert!()`                      |
| Equality Check           | `assert_eq!()`                   |
| Not Equal                | `assert_ne!()`                   |
| Expect Panic             | `#[should_panic]`                |
| Ignore Test              | `#[ignore]`                      |
| Run Tests                | `cargo test`                     |
| Run One Test             | `cargo test name`                |
| Show Output              | `cargo test -- --nocapture`      |
| Single Thread            | `cargo test -- --test-threads=1` |
| Integration Test Folder  | `tests/`                         |
| Shared Integration Setup | `tests/common/mod.rs`            |
| Private Function Testing | Allowed                          |

This covers all major implementation patterns and test cases introduced in Chapter 11 of *The Rust Programming Language*.  
