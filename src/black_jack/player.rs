use std::fmt;

use crate::black_jack::card::Card;
use crate::black_jack::deck::Deck;

#[derive(Clone, Default)]
pub struct Player {
    pub name: String,
    hand: Vec<Card>,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Player {
    pub fn new(name: String) -> Player {
        let hand = vec![];
        Player { name, hand }
    }

    // fn deal_card(&mut self, card: Card);
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_score(&self) -> u32 {
        //get score of all cards added
        let mut score = self.hand.iter().fold(0, |acc, card| acc + card.score());

        // situation when might want A as 11 or 1
        if score > 21 {
            for card in &self.hand {
                if card.value.as_str() == "A" {
                    score -= 10;
                    if score <= 21 {
                        break;
                    }
                }
            }
        }
        score
    }

    //determine if player has black_jack
    pub fn has_black_jack(&self) -> bool {
        println!("score: {} cards: {}", self.get_score(), self.hand.len());
        return self.get_score() == 21 && self.hand.len() == 2;
    }

    //initial round get from deck
    pub fn initial_r(&mut self, deck: &mut Deck) {
        self.hand.clear();
        let mut cards = deck.get_initial_cards();
        self.hand.append(&mut cards);
    }

    //hit
    pub fn hit(&mut self, deck: &mut Deck) {
        let card = deck.deal_card();
        self.hand.push(card);
    }

    pub fn bust(&self) -> bool {
        self.get_score() > 21
    }
}
