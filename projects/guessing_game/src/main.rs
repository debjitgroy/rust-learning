use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    println!("please input your guess:");
    let mut tries = 10;
    loop {
        println!("you have {tries} tries remaining...");
        tries -= 1;
        if tries < 0 {
            break;
        }
        let mut guess = String::new();
        let res = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("res (number of bytes read) is {res}");
        // let guess: u32 = guess.trim().parse().expect("Should be a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess is {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("The secret number is {secret_number}");
}
