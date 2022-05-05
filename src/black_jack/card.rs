use std::fmt;

#[derive(Clone, Debug)]
pub struct Card {
    pub suit: String,
    pub value: String,
}

impl Card {
    pub fn score(&self) -> u32 {
        match self.value.parse::<u32>() {
            Ok(x) => x,
            Err(_) => self.face_card_score(),
        }
    }

    fn face_card_score(&self) -> u32 {
        match self.value.as_str() {
            "J" => 10,
            "Q" => 10,
            "K" => 10,
            "A" => 11,
            _ => 0,
        }
    }

    pub fn to_string(&self) -> String {
        let pattern = match self.suit.as_str() {
            "Hearts" => "♥",
            "Diamonds" => "♦",
            "Clubs" => "♣",
            "Spades" => "♠",
            _ => "",
        };
        return format!("{} of {}", &self.value, pattern);
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        (&self.suit, &self.value) == (&other.suit, &other.value)
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]

fn test_score_for_numbers() {
    let card = Card {
        suit: "Spades".into(),
        value: "3".into(),
    };
    assert_eq!(card.score(), 3);
}

#[test]
fn test_score_for_face_card() {
    let card = Card {
        suit: "Hearts".into(),
        value: "Q".into(),
    };
    assert_eq!(card.score(), 10);
}

#[test]
fn test_str() {
    let card = Card {
        suit: "Hearts".into(),
        value: "K".into(),
    };
    assert_eq!(card.to_string(), String::from("K of ♥"));
}
