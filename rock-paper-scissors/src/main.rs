use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");
    loop {
        println!("Choose your move:");
        println!("1. Rock");
        println!("2. Paper");
        println!("3. Scissors");
        println!("Q. Quit");

        // Player's input
        let mut player_choice = String::new();
        io::stdin().read_line(&mut player_choice).expect("Failed to read input");

        let player_choice = match player_choice.trim() {
            "1" => "Rock",
            "2" => "Paper",
            "3" => "Scissors",
            "Q" | "q" => {
                "Goodbye!";
                break;
            }
            _ => {
                println!("Invalid choice, please select 1, 2, or 3.");
            }
        };

        // Generate computer's move
        let computer_choice = match rand::thread_rng().gen_range(1..=3) {
            1 => "Rock",
            2 => "Paper",
            3 => "Scissors",
            _ => unreachable!(),
        };

        println!("You chose: {}", player_choice);
        println!("Computer chose: {}", computer_choice);

        // Determine the winner
        match (player_choice, computer_choice) {
            (x, y) if x == y => println!("It's a tie!"),
            ("Rock", "Scissors") | ("Paper", "Rock") | ("Scissors", "Paper") => {
                println!("You win!");
            }
            _ => println!("You lose!"),
        }
    }
}
