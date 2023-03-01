use std::io;
use rand::seq::SliceRandom;

fn main() {
    let words = vec![
        "rust",
        "programming",
        "computer",
        "language",
        "algorithm",
        "software",
        "development",
    ];
    let mut rng = rand::thread_rng();
    let word = words.choose(&mut rng).unwrap().to_lowercase();
    let mut guessed = vec!['_'; word.len()];
    let mut guesses = 0;

    println!("Welcome to the Word Guessing Game!");
    println!("The word is {} letters long.", word.len());

    loop {
        println!("\n{}", guessed.iter().collect::<String>());

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let letter = input.trim().chars().next().unwrap().to_lowercase().next().unwrap();

        if word.contains(letter) {
            for (i, c) in word.chars().enumerate() {
                if c == letter {
                    guessed[i] = letter;
                }
            }
            if !guessed.contains(&'_') {
                println!("Congratulations, you won!");
                break;
            }
        } else {
            guesses += 1;
            println!("Sorry, that letter is not in the word. You have {} guesses remaining.", 6 - guesses);
            if guesses == 6 {
                println!("Sorry, you lost. The word was {}.", word);
                break;
            }
        }
    }
}