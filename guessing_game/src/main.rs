extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Please input your guess");
    let mut guess = String::new();

    loop {
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to readline");

        let secret_number = rand::thread_rng().gen_range(1,101);

        //println!("Secret number is {}", secret_number);
        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small"),
            Ordering::Greater   => println!("Too large"),
            Ordering::Equal     => {
                println!("You Win!");
                break;
            }
        }
    }
}
