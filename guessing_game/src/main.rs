use std::io;
use rand::Rng;

fn main() {
    let mut guess= String::new();
    let secret_number = rand::thread_rng().gen_range(1..10);
    println!("Guess the number!");
    println!("Please input your guess.");
    io::stdin().read_line(&mut guess).expect("Failed to Read Line");
    let int_guess = i32::from_str_radix(guess.trim(), 10).expect("Please type a number!");
    if int_guess == secret_number {
        println!("You win!");
    } else {
        println!("You lose! The secret number was {}", secret_number);
     }

}
