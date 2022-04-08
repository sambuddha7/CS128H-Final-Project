extern crate rand;

use blackjack::card::card::{self, Card};
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}
