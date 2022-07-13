use std::fmt;
use rand::Rng;

#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Black,
}

#[derive(Copy, Clone)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}

#[derive(Copy, Clone)]
pub struct Card {
    pub color: Color,
    pub rank: u8,
    pub suit: Suit,
}

pub fn create_deck() -> Vec<Card> {

    let mut deck = Vec::new();

    for suit_num in 0..4 {
        let mut suit = Suit::Heart;
        let mut color = Color::Red;

        match suit_num {
            1 => { suit = Suit::Diamond; },
            2 => { suit = Suit::Club; color = Color::Black },
            3 => { suit = Suit::Spade; color = Color::Black },
            _ => {},
        }

        for rank in 1..14 {
            deck.push(Card {
                color,
                rank,
                suit,
            });
        }
    }

    deck
}

pub fn draw(mut deck: Vec<Card>) -> (Vec<Card>, Card) {
    let card = deck.remove(0);
    (deck, card)
}

pub fn draw_randomly(mut deck: Vec<Card>) -> (Vec<Card>, Card) {
    let rand_num = rand::thread_rng().gen_range(0..deck.len());
    let card = deck.remove(rand_num);
    (deck, card)
}

pub fn shuffle(mut deck: Vec<Card>) -> Vec<Card> {
    let mut card = Card { color: Color::Red, rank: 0, suit: Suit::Heart };
    let mut shuffled_deck = Vec::new();
    let deck_length = deck.len();
    let mut c = 0;

    while c < deck_length {
        (deck, card) = draw_randomly(deck);
        shuffled_deck.push(card);
        c += 1;
    }

    shuffled_deck
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Black => write!(f, "Black"),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Heart => write!(f, "Heart"),
            Suit::Diamond => write!(f, "Diamond"),
            Suit::Club => write!(f, "Club"),
            Suit::Spade => write!(f, "Spade"),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = String::new();
        match self.rank {
            1 => text = "Ace".to_string(),
            2 => text = "Two".to_string(),
            3 => text = "Three".to_string(),
            4 => text = "Four".to_string(),
            5 => text = "Five".to_string(),
            6 => text = "Six".to_string(),
            7 => text = "Seven".to_string(),
            8 => text = "Eight".to_string(),
            9 => text = "Nine".to_string(),
            10 => text = "Ten".to_string(),
            11 => text = "Jack".to_string(),
            12 => text = "Queen".to_string(),
            13 => text = "King".to_string(),
            _ => text = "Joker".to_string(),
        }
        write!(f, "{} {} of {}s", self.color, text, self.suit)
    }
}