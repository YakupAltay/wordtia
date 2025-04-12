use celestia_types::nmt::Namespace;
use anyhow::Result;

/// Namespace bytes must be exactly 10 bytes long for Celestia's v0 namespace format.
/// This represents the ASCII values for the string "WORDTIATIA".
const WORDTIA_NAMESPACE_BYTES: [u8; 10] = [
    87, 79, 82, 68, 84, 73, 65, 84, 73, 65 // "W", "O", "R", "D", "T", "I", "A", "T", "I", "A"
];

/// Returns a fixed-size v0 namespace for the game, constructed from a 10-byte constant.
pub fn get_wordtia_namespace() -> Result<Namespace> {
    // `const_v0` constructs a namespace without runtime validation
    let namespace = Namespace::const_v0(WORDTIA_NAMESPACE_BYTES);
    Ok(namespace)
}

// Constants for game configuration and networking

/// Local Celestia RPC endpoint (adjustable for different environments or deployments)
pub const CELESTIA_RPC_URL: &str = "http://localhost:10101";

/// Maximum number of allowed guesses in the game
pub const MAX_ATTEMPTS: usize = 6;

/// Required length for a valid guess word
pub const WORD_LENGTH: usize = 5;

/// A list of valid 5-letter words that can be randomly selected as the secret word
pub const WORDLIST: &[&str] = &[
    "apple", "crane", "spine", "glide", "brave", "sword", "knock", "grind", "flame",
    "pride", "brisk", "plant", "charm", "liver", "earth", "vapor", "shiny", "cheer",
    "crown", "frost", "grasp", "hazel", "latch", "mirth", "plush", "quill", "reign",
];
