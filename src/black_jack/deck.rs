use crate::black_jack::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::vec;
#[derive(Clone, Default)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    //  make new deck of cards
    pub fn new(deck_n: usize) -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..deck_n {
            cards.append(&mut Deck::make_deck());
        }
        Deck { cards }
    }
    // shuffle
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    // make deck w/ deck counts
    fn make_deck() -> Vec<Card> {
        let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
        let numbers = vec![
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];
        let mut cards = vec![];
        for suit in suits {
            for num in &numbers {
                let card = Card {
                    suit: suit.into(),
                    value: num.to_string(),
                };
                cards.push(card);
            }
        }
        cards
    }

    //draw next card
    pub fn deal_card(&mut self) -> Card {
        return self.cards.pop().unwrap();
    }

    //initial round card
    pub fn get_initial_cards(&mut self) -> Vec<Card> {
        vec![self.deal_card(), self.deal_card()]
    }
}
