extern crate rand;
pub mod deck;

use std::io;
use std::str;
use deck::Deck;
use deck::Card;

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    //fn value(&self) -> i32 {
        ////println!("{:?}", self.cards)
    //}
}

fn main() {
    println!("#########################################");
    println!("######### Welcome to Blackjack! #########");
    println!("#########################################");

    // shuffle a new deck
    let mut deck = Deck::new();

    // will wanna mut so we can hit
    let mut player_hand = deck.deal(2);
    println!("\n   You: {:?}", player_hand);

    let computer_hand = deck.deal(2);
    println!("Dealer: [{:?}, X]", computer_hand[0]);

    let mut bust = false;
    loop {
        println!("Would you like to hit or stand?");

        let mut response = String::new();

        io::stdin().read_line(&mut response)
            .expect("Failed to read line");

        if response.trim() == "hit" {
            player_hand.extend(deck.deal(1));
            println!("Your hand is now: {:?}", player_hand);
        } else if response.trim() == "stand" {
            break;
        } else {
            println!("I could not understand your response. Try again.");
        }
    }
}
