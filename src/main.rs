extern crate rand;
pub mod deck;

use std::io;
use std::str;
use deck::Deck;
use deck::Card;

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn view(&self) {
        println!("   You: {:?}", self.cards);
        println!(" Value: {}", self.value());
    }

    // TODO: fix this cause it makes sense
    //fn hit(&self) {
        //self.cards.extend(deck.deal(1));
    //}

    fn value(&self) -> i32 {
        let mut sum = 0;
        for card in &self.cards {
            sum += card.value();
        }
        // Players choice, make Ace = 1 only if we go over 21
        if sum > 21 && self.cards.iter().any (|x| x.value == deck::Value::Ace) {
            sum -= 10;
        }
        sum
    }
}

fn main() {
    println!("#########################################");
    println!("######### Welcome to Blackjack! #########");
    println!("#########################################");

    // shuffle a new deck
    let mut deck = Deck::new();

    // will wanna mut so we can hit
    let mut player_hand = Hand {
        cards: deck.deal(2),
    };
    player_hand.view();

    let computer_hand = deck.deal(2);
    println!("Dealer: [{:?}, X]", computer_hand[0]);

    let mut bust = false;
    loop {
        println!("\nWould you like to hit or stand?");

        let mut response = String::new();

        io::stdin().read_line(&mut response)
            .expect("Failed to read line");

        if response.trim() == "hit" {
            player_hand.cards.extend(deck.deal(1));
            player_hand.view();
        } else if response.trim() == "stand" {
            break;
        } else {
            println!("I could not understand your response. Try again.");
        }
    }
}
