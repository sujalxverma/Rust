use std::io;
// #[derive(Debug)]
//      take number as input
//     let mut number = String::new();
//     println!("Enter the number : ");
//     io::stdin().read_line(&mut number).expect("Failed");
//     let num :i32 = number.trim().parse().expect("Enter the number");

// use std::io::{self, Write};

fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
    if divisor == 0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

fn main() {
    match checked_division(11, 2) {
        Some(result) => println!("Result: {}", result),
        None => println!("Division by zero!"),
    }
}
