use rand::{rng, seq::SliceRandom};

use std::vec;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards
            .split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();
    // Probably need to add error handling!!!!
    let cards = deck.deal(300);

    println!("Here is your hand: {cards:#?}");
    println!("Here is your deck: {deck:#?}");
}