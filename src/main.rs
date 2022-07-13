pub mod card;

use std::io;

use crate::card::{ Color, Suit, Card, create_deck, draw, shuffle };

fn get_score(hand: &Vec<Card>) -> u8 {
    let mut score = 0;
    for card in hand {
        match card.rank {
            2..=10 => { score += card.rank },
            11.. => { score += 10 },
            _ => {
                score += 11;
                if score > 21 {
                    score -= 10;
                }
            }
        }
    }
    score
}

fn print_hand(hand: &Vec<Card>) {
    for card in hand {
        println!("{}", card);
    }
}

fn main() {

    let mut deck = shuffle(create_deck());
    let mut card = Card { color: Color::Red, rank: 0, suit: Suit::Heart };

    (deck, card) = draw(deck);
    let mut players_hand = vec!(card);

    (deck, card) = draw(deck);
    let mut dealers_hand = vec!(card);

    (deck, card) = draw(deck);
    players_hand.push(card);

    (deck, card) = draw(deck);
    dealers_hand.push(card);

    println!("The game has started...");

    println!("\nYour hand:");
    print_hand(&players_hand);
    println!("Your score: {}", get_score(&players_hand));

    println!("\nThe dealer's hand:");
    println!("An unknown card");
    println!("{}", dealers_hand[1]);
    card = dealers_hand[1];
    let mut score = 0;
    match card.rank {
        2..=10 => { score += card.rank },
        11.. => { score += 10 },
        _ => {
            score += 11;
            if score > 21 {
                score -= 10;
            }
        }
    }
    println!("Dealer's score: ? + {}", score);

    let mut stay = false;
    while !stay {
        let mut choice = String::new();
        println!("\nHit or stay? (h/s): ");
        io::stdin().read_line(&mut choice).expect("Failed to read line.");
        choice = choice.to_lowercase();

        if choice.contains('h') {
            (deck, card) = draw(deck);
            players_hand.push(card);
            println!("You drew a {}", card);

            println!("\nYour hand:");
            print_hand(&players_hand);
            println!("Your score: {}", get_score(&players_hand));

            if get_score(&players_hand) > 21 {
                break;
            }
        } else if choice.contains('s') {
            stay = true;
        }
    }

    if get_score(&players_hand) > 21 {
        println!("\nYou lose!");
    } else {
        let mut stay = false;
        while !stay {
            let score = get_score(&dealers_hand);
            if score < 17 {
                (deck, card) = draw(deck);
                dealers_hand.push(card);
            } else {
                stay = true;
            }
        }

        let players_score = get_score(&players_hand);
        let dealers_score = get_score(&dealers_hand);

        println!("\nThe dealer's hand:");
        print_hand(&dealers_hand);
        println!("The dealer's score: {}", dealers_score);

        if players_score > dealers_score || dealers_score > 21{
            println!("You win!");
        } else if players_score < dealers_score {
            println!("You lose!");
        } else {
            println!("You tie!");
        }
    }
}