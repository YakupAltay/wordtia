use chrono::Utc;
use rand::seq::SliceRandom;
use serde::Serialize;
use celestia_rpc::{Client, TxConfig};

use crate::constants::{CELESTIA_RPC_URL, MAX_ATTEMPTS, WORDLIST};
use crate::utils::{hash_word, read_guess, is_valid_guess, format_feedback_grid};
use crate::celestia::{prepare_blob, submit_blob_to_celestia, log_submission_result};

#[derive(Serialize)]
pub struct GameResult {
    word_hash: String,
    guess_hashes: Vec<String>,
    success: bool,
    timestamp: String,
}

#[tokio::main]
pub async fn run_game() {
    // Randomly select a secret word from the predefined word list
    let secret_word = WORDLIST.choose(&mut rand::thread_rng()).unwrap().to_string();
    let word_hash = hash_word(&secret_word);

    let mut guesses = Vec::new();
    let mut guess_hashes = Vec::new();
    let mut feedback_history = Vec::new();

    // Loop until the player runs out of attempts or guesses correctly
    while guesses.len() < MAX_ATTEMPTS {
        // Prompt and validate user input
        let guess = loop {
            match read_guess() {
                Some(g) if is_valid_guess(&g) => break g,
                _ => {
                    println!("❌ Guess must be exactly 5 alphabetic letters.");
                    continue;
                }
            }
        };

        // Store valid guess and its hash
        guesses.push(guess.clone());
        guess_hashes.push(hash_word(&guess));

        // Generate and print visual feedback for the guess
        let (boxes, letters) = format_feedback_grid(&guess, &secret_word);
        feedback_history.push((boxes.clone(), letters.clone()));
        println!("{boxes}\n{letters}\n");

        // End game if correct word is guessed
        if guess == secret_word {
            break;
        }
    }

    // Determine game success
    let success = guesses.last().map(|g| g == &secret_word).unwrap_or(false);

    if success {
        println!("🎉 You guessed the word!");
    } else {
        println!("💀 You didn't guess the word.");
    }

    // Display final feedback board
    println!("\n📊 Final Board:");
    for (boxes, letters) in &feedback_history {
        println!("{boxes}\n{letters}\n");
    }

    // Construct game result data for attestation
    let result = GameResult {
        word_hash,
        guess_hashes,
        success,
        timestamp: Utc::now().to_rfc3339(),
    };

    // Serialize result into a blob and submit it to Celestia
    let blob = prepare_blob(&result).expect("Failed to prepare blob");
    let client = Client::new(CELESTIA_RPC_URL, None)
        .await
        .expect("Failed to connect to Celestia RPC");
    let config = TxConfig::default();

    // Submit the blob and log the result
    match submit_blob_to_celestia(&client, blob, config).await {
        Ok(response) => log_submission_result(&response),
        Err(e) => eprintln!("❌ Submission failed: {}", e),
    }
}
