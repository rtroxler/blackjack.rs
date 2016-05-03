//extern crate rand;
use rand::{thread_rng, Rng};
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
enum Suit {
    Diamonds, Hearts, Spades, Clubs
}

#[allow(dead_code)]
#[derive(Debug)]
enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

pub struct Card(Value, Suit);

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {:?}", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cards: Deck::shuffled_cards() }
    }

    pub fn deal(&mut self, num: i32) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::new();
        for _ in 0..num {
            result.push(self.cards.pop().unwrap());
        }
        result
    }

    fn shuffled_cards() -> Vec<Card> {
        let mut cards = vec![
            Card(Value::Ace,    Suit::Diamonds),
            Card(Value::Two,    Suit::Diamonds),
            Card(Value::Three,  Suit::Diamonds),
            Card(Value::Four,   Suit::Diamonds),
            Card(Value::Five,   Suit::Diamonds),
            Card(Value::Six,    Suit::Diamonds),
            Card(Value::Seven,  Suit::Diamonds),
            Card(Value::Eight,  Suit::Diamonds),
            Card(Value::Nine,   Suit::Diamonds),
            Card(Value::Ten,    Suit::Diamonds),
            Card(Value::Jack,   Suit::Diamonds),
            Card(Value::Queen,  Suit::Diamonds),
            Card(Value::King,   Suit::Diamonds),
            Card(Value::Ace,    Suit::Spades),
            Card(Value::Two,    Suit::Spades),
            Card(Value::Three,  Suit::Spades),
            Card(Value::Four,   Suit::Spades),
            Card(Value::Five,   Suit::Spades),
            Card(Value::Six,    Suit::Spades),
            Card(Value::Seven,  Suit::Spades),
            Card(Value::Eight,  Suit::Spades),
            Card(Value::Nine,   Suit::Spades),
            Card(Value::Ten,    Suit::Spades),
            Card(Value::Jack,   Suit::Spades),
            Card(Value::Queen,  Suit::Spades),
            Card(Value::King,   Suit::Spades),
            Card(Value::Ace,    Suit::Clubs),
            Card(Value::Two,    Suit::Clubs),
            Card(Value::Three,  Suit::Clubs),
            Card(Value::Four,   Suit::Clubs),
            Card(Value::Five,   Suit::Clubs),
            Card(Value::Six,    Suit::Clubs),
            Card(Value::Seven,  Suit::Clubs),
            Card(Value::Eight,  Suit::Clubs),
            Card(Value::Nine,   Suit::Clubs),
            Card(Value::Ten,    Suit::Clubs),
            Card(Value::Jack,   Suit::Clubs),
            Card(Value::Queen,  Suit::Clubs),
            Card(Value::King,   Suit::Clubs),
            Card(Value::Ace,    Suit::Hearts),
            Card(Value::Two,    Suit::Hearts),
            Card(Value::Three,  Suit::Hearts),
            Card(Value::Four,   Suit::Hearts),
            Card(Value::Five,   Suit::Hearts),
            Card(Value::Six,    Suit::Hearts),
            Card(Value::Seven,  Suit::Hearts),
            Card(Value::Eight,  Suit::Hearts),
            Card(Value::Nine,   Suit::Hearts),
            Card(Value::Ten,    Suit::Hearts),
            Card(Value::Jack,   Suit::Hearts),
            Card(Value::Queen,  Suit::Hearts),
            Card(Value::King,   Suit::Hearts),
            ];
        thread_rng().shuffle(&mut cards);
        cards
    }
}
