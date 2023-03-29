mod card;
mod game;

use crate::game::create_deck;

fn main() {
    let deck = create_deck();
    for card in &deck {
        println!("{}", card);
    }
}
