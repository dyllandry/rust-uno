mod card;
mod game;

use crate::game::create_deck;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let mut deck = create_deck();
    deck.shuffle(&mut thread_rng());
    for card in &deck {
        println!("{}", card);
    }
}
