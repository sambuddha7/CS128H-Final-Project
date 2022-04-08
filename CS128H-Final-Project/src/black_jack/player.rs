use super::*;
use blackjack::card::card::{self, Card};

pub struct Player {
    pub name: String,
    hand: Vec<Card>,
}

pub struct Dealer {
    pub name: String,
    hand: Vec<Card>,
}

pub trait Person {
    //  fn new(name: &str) -> Self;
    fn deal_card(&mut self, card: Card);
    fn get_hand(&self) -> &Vec<Card>;
    fn get_name(&self) -> &str;
}
