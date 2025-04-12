mod constants;
mod utils;
mod game;
mod celestia;

use game::run_game;

fn main() {
    // Game introduction message
    println!("🔐 Welcome to WordTia! (Wordle with Celestia underneath ✨)");
    println!("Guess the secret 5-letter word. You have 6 attempts.\n");

    // Start the async game logic (handled inside run_game with #[tokio::main])
    run_game();
}
