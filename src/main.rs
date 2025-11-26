use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::fs::{OpenOptions, read_to_string};
use std::cmp::Reverse;

struct Word {
    japanese: &'static str,
    english: &'static str,
}

fn main() {
    // Words database
    let words = vec![
        Word { japanese: "犬", english: "Dog" },
        Word { japanese: "猫", english: "Cat" },
        Word { japanese: "水", english: "Water" },
        Word { japanese: "火", english: "Fire" },
        Word { japanese: "空", english: "Sky" },
    ];

    let mut score = 0;
    let start = Instant::now();
    let game_duration = Duration::from_secs(30); // 30-second game

    println!("Japanese Vocabulary Game! You have 30 seconds.\n");

    while Instant::now() - start < game_duration {
        // Pick correct and wrong word
        let correct_word = words.choose(&mut rand::thread_rng()).unwrap();
        let mut wrong_word = words.choose(&mut rand::thread_rng()).unwrap();
        while wrong_word.english == correct_word.english {
            wrong_word = words.choose(&mut rand::thread_rng()).unwrap();
        }

        // Randomize options
        let options = if rand::random() {
            vec![correct_word.english, wrong_word.english]
        } else {
            vec![wrong_word.english, correct_word.english]
        };

        // Show question
        println!("What is '{}' in English?", correct_word.japanese);
        println!("1) {}", options[0]);
        println!("2) {}", options[1]);
        print!("Your choice (1 or 2): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice = input.trim().parse::<usize>().unwrap_or(0);

        if choice >= 1 && choice <= 2 {
            if options[choice - 1] == correct_word.english {
                println!("Correct! +1 point\n");
                score += 1;
            } else {
                println!("Wrong! -1 point\n");
                score -= 1;
            }
        } else {
            println!("Invalid choice, skipping!\n");
        }
    }

    println!("Time's up! Your score: {}", score);

    // Update leaderboard
    let leaderboard_file = "leaderboard.txt";
    let mut leaderboard = vec![];

    if let Ok(contents) = read_to_string(leaderboard_file) {
        for line in contents.lines() {
            if let Ok(val) = line.parse::<i32>() {
                leaderboard.push(val);
            }
        }
    }

    leaderboard.push(score);
    leaderboard.sort_by_key(|&s| Reverse(s)); // highest score first

    println!("\nLeaderboard:");
    for (i, &s) in leaderboard.iter().enumerate().take(5) {
        println!("{}. {}", i + 1, s);
    }

    // Save updated leaderboard
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(leaderboard_file)
        .unwrap();

    for s in leaderboard {
        writeln!(file, "{}", s).unwrap();
    }
}
