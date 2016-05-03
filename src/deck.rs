//extern crate rand;
use rand::{thread_rng, Rng};
use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Suit {
    Diamonds, Hearts, Spades, Clubs
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Value {
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

pub struct Card {
    pub value: Value,
    suit: Suit,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} - {:?}", self.value, self.suit)
    }
}

impl Card {
    pub fn value(&self) -> i32 {
        match self.value {
            Value::Ace => 11,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
        }
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
        // this stinks, but it'd be unsafe to try to loop for each and dynamically build this?
        // maybe
        let mut cards = vec![
            Card{ value: Value::Ace,    suit: Suit::Diamonds },
            Card{ value: Value::Two,    suit: Suit::Diamonds },
            Card{ value: Value::Three,  suit: Suit::Diamonds },
            Card{ value: Value::Four,   suit: Suit::Diamonds },
            Card{ value: Value::Five,   suit: Suit::Diamonds },
            Card{ value: Value::Six,    suit: Suit::Diamonds },
            Card{ value: Value::Seven,  suit: Suit::Diamonds },
            Card{ value: Value::Eight,  suit: Suit::Diamonds },
            Card{ value: Value::Nine,   suit: Suit::Diamonds },
            Card{ value: Value::Ten,    suit: Suit::Diamonds },
            Card{ value: Value::Jack,   suit: Suit::Diamonds },
            Card{ value: Value::Queen,  suit: Suit::Diamonds },
            Card{ value: Value::King,   suit: Suit::Diamonds },
            Card{ value: Value::Ace,    suit: Suit::Spades },
            Card{ value: Value::Two,    suit: Suit::Spades },
            Card{ value: Value::Three,  suit: Suit::Spades },
            Card{ value: Value::Four,   suit: Suit::Spades },
            Card{ value: Value::Five,   suit: Suit::Spades },
            Card{ value: Value::Six,    suit: Suit::Spades },
            Card{ value: Value::Seven,  suit: Suit::Spades },
            Card{ value: Value::Eight,  suit: Suit::Spades },
            Card{ value: Value::Nine,   suit: Suit::Spades },
            Card{ value: Value::Ten,    suit: Suit::Spades },
            Card{ value: Value::Jack,   suit: Suit::Spades },
            Card{ value: Value::Queen,  suit: Suit::Spades },
            Card{ value: Value::King,   suit: Suit::Spades },
            Card{ value: Value::Ace,    suit: Suit::Clubs },
            Card{ value: Value::Two,    suit: Suit::Clubs },
            Card{ value: Value::Three,  suit: Suit::Clubs },
            Card{ value: Value::Four,   suit: Suit::Clubs },
            Card{ value: Value::Five,   suit: Suit::Clubs },
            Card{ value: Value::Six,    suit: Suit::Clubs },
            Card{ value: Value::Seven,  suit: Suit::Clubs },
            Card{ value: Value::Eight,  suit: Suit::Clubs },
            Card{ value: Value::Nine,   suit: Suit::Clubs },
            Card{ value: Value::Ten,    suit: Suit::Clubs },
            Card{ value: Value::Jack,   suit: Suit::Clubs },
            Card{ value: Value::Queen,  suit: Suit::Clubs },
            Card{ value: Value::King,   suit: Suit::Clubs },
            Card{ value: Value::Ace,    suit: Suit::Hearts },
            Card{ value: Value::Two,    suit: Suit::Hearts },
            Card{ value: Value::Three,  suit: Suit::Hearts },
            Card{ value: Value::Four,   suit: Suit::Hearts },
            Card{ value: Value::Five,   suit: Suit::Hearts },
            Card{ value: Value::Six,    suit: Suit::Hearts },
            Card{ value: Value::Seven,  suit: Suit::Hearts },
            Card{ value: Value::Eight,  suit: Suit::Hearts },
            Card{ value: Value::Nine,   suit: Suit::Hearts },
            Card{ value: Value::Ten,    suit: Suit::Hearts },
            Card{ value: Value::Jack,   suit: Suit::Hearts },
            Card{ value: Value::Queen,  suit: Suit::Hearts },
            Card{ value: Value::King,   suit: Suit::Hearts },
        ];
        // shuffle me
        thread_rng().shuffle(&mut cards);
        cards
    }
}
