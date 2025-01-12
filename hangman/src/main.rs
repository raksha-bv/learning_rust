use std::io::{ self, Write };
use std::collections::HashSet;

fn main() {
    println!("Welcome to Hangman!");

    // The word to guess
    let secret_word = "rustacean";
    let secret_word_chars: Vec<char> = secret_word.chars().collect();

    // Set of guessed letters
    let mut guessed_letters = HashSet::new();
    let mut attempts_remaining = 7;

    loop {
        // Display the current progress of the word
        let display_word: String = secret_word_chars
            .iter()
            .map(|&c| if guessed_letters.contains(&c) { c } else { '_' })
            .collect();
        println!("\nWord: {}", display_word);

        // Check if the user has guessed the word
        if display_word == secret_word {
            println!("Congratulations! You guessed the word!");
            break;
        }

        // Check if the user is out of attempts
        if attempts_remaining == 0 {
            println!("You've run out of attempts! The word was '{}'. Better luck next time!", secret_word);
            break;
        }

        // Prompt for a guess
        print!("Enter a letter ({} attempts remaining): ", attempts_remaining);
        io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess = guess.trim().chars().next();

        if let Some(letter) = guess {
            if guessed_letters.contains(&letter) {
                println!("You already guessed '{}'. Try a different letter.", letter);
            } else if secret_word_chars.contains(&letter) {
                println!("Good guess! '{}' is in the word.", letter);
                guessed_letters.insert(letter);
            } else {
                println!("Wrong guess! '{}' is not in the word.", letter);
                guessed_letters.insert(letter);
                attempts_remaining -= 1;
            }
        } else {
            println!("Please enter a valid letter.");
        }
    }
}
