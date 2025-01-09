use rand::Rng;
use std::io;

fn main() {
    let truths = vec![
        "Who is your crush?",
        "Which is your favorite programming language?",
        "What is your favorite food?"
    ];
    let dares = vec![
        "Climb the tree in 30 seconds",
        "Belly dance for 15 seconds",
        "Sing a song of your choice"
    ];
    println!("Welcome to Truth or Dare!");
    println!("Enter the number of players:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num_players: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number. Exiting...");
            return;
        }
    };

    let mut players = Vec::new();
    for i in 1..=num_players {
        println!("Enter the name of player {}:", i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read input");
        players.push(name.trim().to_string());
    }
    loop {
        let random_player = &players[rand::thread_rng().gen_range(0..players.len())];
        println!("\nIt's {}'s turn!", random_player);

        println!("Choose an option: (truth/dare/exit)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().to_lowercase();

        match choice.as_str() {
            "truth" => {
                let random_index = rand::thread_rng().gen_range(0..truths.len());
                println!("Truth: {}", truths[random_index]);
            }
            "dare" => {
                let random_index = rand::thread_rng().gen_range(0..dares.len());
                println!("Dare: {}", dares[random_index]);
            }
            "exit" => {
                println!("Thanks for playing! Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice. Please type 'truth', 'dare', or 'exit'.");
            }
        }
    }
}
