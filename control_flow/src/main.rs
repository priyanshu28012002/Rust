fn main() {
    // if else

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //  let number = 3;
    // if number {
    //     println!("number was three");
    // }

    let number = 3;
    if number == 3 {
        println!("number was three");
    }

    let number = 21;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // if as expresion
                                                // let number = if condition { 5 } else { "six" };
    println!("The value of number is: {number}");

    // Loop

    let mut num = 10;
    loop {
        println!("The value of number is: {num}");

        num += 1;
        if num > 12 {
            break;
        }
    }

    // Returning Values from Loops

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Disambiguating with Loop Labels
    println!("Disambiguating with Loop Labels ");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Streamlining Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");


    for number in (1..14).rev() {
        println!("{number}!");
    }
}
