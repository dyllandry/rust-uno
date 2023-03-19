mod card;

use crate::card::{Card, Color};

fn main() {
    let mut card = Card::default();
    card.number = Some(5);
    card.color = Some(Color::Red);
    println!("{}", card);
}

