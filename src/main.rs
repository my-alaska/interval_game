use std::process;

fn main() {
    if let Err(e) = interval_game::game_loop() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
