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

    fn hit(&mut self, deck: &mut Deck) {
        self.cards.extend(deck.deal(1));
        self.view();
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
        if sum > 21 && self.cards.iter().any (|x| x.is_ace() ) {
            sum -= 10;
        }
        sum
    }
}

#[derive(Debug)]
pub struct Game {
    deck:          Deck,
    computer_hand: Hand,
    player_hand:   Hand
}

impl Game {
    pub fn new() -> Game {
        let mut deck = Deck::new();
        let (computer_hand, player_hand) = initialize_hands(&mut deck);

        Game {
            deck: deck,
            computer_hand: computer_hand,
            player_hand: player_hand
        }
    }

    pub fn start(&mut self) {
        let mut bust = false;
        loop {
            println!("\nWould you like to hit or stand?");

            let mut response = String::new();
            io::stdin().read_line(&mut response)
                .expect("Failed to read line");

            let hit_responses = ["hit", "h"];
            let stand_responses = ["stand", "s"];

            if hit_responses.contains(&response.trim()) {
                self.player_hand.hit(&mut self.deck);
                if self.player_hand.busted() {
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
            self.computer_hand.view();
            while self.computer_hand.value() < 17 {
                self.computer_hand.hit(&mut self.deck);
            }
            if self.computer_hand.busted() || self.computer_hand.value() < self.player_hand.value() {
                println!("YOU WIN!");
            } else if self.computer_hand.value() == self.player_hand.value() {
                println!("It's a tie!");
            } else {
                println!("You lost. :( ");
            }
        } else {
            println!("Sorry dude.")
        }
    }
}


fn initialize_hands(deck: &mut Deck) -> (Hand, Hand){
    let computer_hand = Hand { cards: deck.deal(2), };
    println!("Dealer: [{:?}, X]", computer_hand.cards[0]);

    let player_hand = Hand { cards: deck.deal(2), };
    println!("You:");
    player_hand.view();

    ( computer_hand, player_hand )
}
