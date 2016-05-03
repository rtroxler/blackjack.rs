extern crate rand;
pub mod deck;

use std::io;
use std::str;
use deck::Deck;

fn main() {
    println!("#########################");
    println!("# Welcome to Blackjack! #");
    println!("#########################");
    // need to have a hand, composed of two cards

    // shuffle a new deck
    let mut deck = Deck::new();

    // will wanna mut
    let hand = deck.deal(2);
    println!("\nHere's your hand: {:?}", hand);

    loop {
        println!("Would you like to hit or stand?");

        let mut response = String::new();

        io::stdin().read_line(&mut response)
            .expect("Failed to read line");

        if response.trim() == "hit" {
            println!(" Finna hit");
        } else if response.trim() == "stand" {
            println!(" Finna stand");
        } else {
            println!("I could not understand your response. Try again.");
        }
    }
}
