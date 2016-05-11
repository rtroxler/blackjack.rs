extern crate rand;
pub mod deck;

use std::io;
use deck::Deck;
use deck::Card;

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn view(&self) {
        println!(" Cards: {:?}", self.cards);
        println!(" Value: {}", self.value());
    }

    fn busted(&self) -> bool {
        self.value() > 21
    }

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

    let mut computer_hand = Hand {
        cards: deck.deal(2),
    };
    println!("Dealer: [{:?}, X]", computer_hand.cards[0]);


    let mut player_hand = Hand {
        cards: deck.deal(2),
    };
    println!("You:");
    player_hand.view();

    let mut bust = false;
    loop {
        println!("\nWould you like to hit or stand?");

        let mut response = String::new();

        io::stdin().read_line(&mut response)
            .expect("Failed to read line");

        let hit_responses = ["hit", "h"];
        let stand_responses = ["stand", "s"];
        if hit_responses.contains(&response.trim()) {
            player_hand.cards.extend(deck.deal(1));
            player_hand.view();
            if player_hand.busted() {
                println!("Aw shucks, we busted.. :(");
                bust = true;
                break;
            }
        } else if stand_responses.contains(&response.trim()){
            break;
        } else {
            println!("I could not understand your response. Try again.");
        }
    }

    if !bust {
        println!("Computer:");
        computer_hand.view();
        while computer_hand.value() < 17 {
            computer_hand.cards.extend(deck.deal(1));
            computer_hand.view();
        }
        if computer_hand.busted() || computer_hand.value() < player_hand.value() {
            println!("YOU WIN!");
        } else if computer_hand.value() == player_hand.value() {
            println!("It's a tie!");
        } else {
            println!("You lost. :( ");
        }
    } else {
        println!("Sorry dude.")
    }
    play_again();
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
