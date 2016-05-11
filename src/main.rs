extern crate rand;
pub mod deck;
pub mod game;

use std::io;
use game::Game;

fn display_welcome() {
    println!("#########################################");
    println!("######### Welcome to Blackjack! #########");
    println!("#########################################");
}

fn play_again() {
    println!("\nWould you like to play again?");

    let mut play_again = String::new();
    io::stdin().read_line(&mut play_again)
        .expect("Failed to read line");

    let yes_responses = ["yes", "Y", "y"];
    if yes_responses.contains(&play_again.trim()) {
        println!("\n");
        main();
    }
}

fn main() {
    display_welcome();
    let mut game = Game::new();

    game.start();

    play_again();
}

