//guessing game
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to the Guessing Game");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    //println!("{}", secret_number);
    loop {
        println!("Please input your guess");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("Your guessed:{}", guess);
        let guess: u32 = guess.trim().parse().expect("Type an integer"); //converting to integer data type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
