use sha2::{Digest, Sha256};
use std::io::{self, Write};

use crate::constants::WORD_LENGTH;

/// Hashes a word using SHA-256 and returns the hexadecimal string representation.
/// This is used to commit to the secret word and track guesses without revealing them.
pub fn hash_word(word: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(word.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Generates a visual feedback grid comparing a guess to the secret word.
/// - 🟩 indicates correct letter in the correct position.
/// - 🟨 indicates correct letter in the wrong position.
/// - ⬜ indicates incorrect letter.
pub fn format_feedback_grid(guess: &str, secret: &str) -> (String, String) {
    let mut boxes = String::new();
    let mut letters = String::new();

    for (g_char, s_char) in guess.chars().zip(secret.chars()) {
        let box_emoji = if g_char == s_char {
            "🟩" // Correct letter and position
        } else if secret.contains(g_char) {
            "🟨" // Correct letter, wrong position
        } else {
            "⬜" // Letter not in the word
        };

        boxes.push_str(&format!("{} ", box_emoji));
        letters.push_str(&format!("{}  ", g_char.to_ascii_uppercase()));
    }

    (boxes.trim_end().to_string(), letters.trim_end().to_string())
}

/// Prompts the user to enter a guess, then reads and normalizes the input.
/// Returns `Some` guess in lowercase if successful, or `None` on error.
pub fn read_guess() -> Option<String> {
    print!("> ");
    io::stdout().flush().ok()?; // Ensure prompt is printed before reading input

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).ok()?; // Read input line from stdin
    Some(guess.trim().to_lowercase()) // Normalize and return
}

/// Validates a guess:
/// - It must be exactly `WORD_LENGTH` characters.
/// - All characters must be ASCII alphabetic letters.
pub fn is_valid_guess(guess: &str) -> bool {
    guess.len() == WORD_LENGTH && guess.chars().all(|c| c.is_ascii_alphabetic())
}
