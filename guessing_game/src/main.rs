use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut num_guess = 0;

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        num_guess += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        if num_guess > 5 {
            println!("You lost! The number was {}.", secret_number);
            break;
        }
    }
}
