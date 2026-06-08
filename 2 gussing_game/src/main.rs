use rand::Rng;
use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    print!("Secret number: {secret_number}\n");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed: {guess}");

        // let guess: u32 = guess.trim().parse().expect("Fail to Parse!"); // 45/n //.trim() removes whitespace and enter /n , .parse() converts string to number, .expect() handles error if parse fails
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(err)=>{
                println!("Error => {}" ,err );
                continue;
            }
        }; // return result; 

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                print!("You won \n");
                break;
            },
            Ordering::Greater => print!("Too Big \n"),
            Ordering::Less => print!("Too Small \n"),
        }
    }
}
