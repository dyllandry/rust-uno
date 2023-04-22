use rand::{seq::SliceRandom, thread_rng};

use crate::{
    card::{Card, Color, DrawEffect, TurnEffect},
    user_input::Input,
};

/**
 This mod is for game rules.
*/

pub struct Uno {
    players: Vec<Player>,
    current_player_index: i32,
    deck: Vec<Card>,
    discard: Vec<Card>,
    index_of_wild_card_being_played: Option<i32>,
    turn_order: TurnOrder,
}

impl Uno {
    pub fn new(player_count: i32) -> Self {
        let mut game = Uno {
            current_player_index: 0,
            players: Vec::new(),
            deck: Vec::new(),
            discard: Vec::new(),
            index_of_wild_card_being_played: None,
            turn_order: TurnOrder::Forward,
        };

        game.deck = create_deck();
        game.deck.shuffle(&mut thread_rng());

        for _ in 0..player_count {
            let mut player = Player::default();
            for _ in 0..7 {
                let card = game.deck.pop().unwrap();
                player.hand.push(card);
            }
            game.players.push(player);
        }
        game
    }

    pub fn input(&mut self, input: Input) {
        let mut played_card: Option<Card> = None;

        // We tell what the player is doing based on if they input text or a number.
        match input {
            Input::Text(input_text) => {
                let current_player = &mut self.players[self.current_player_index as usize];
                // Pick color of wild card
                if let Some(wild_index) = self.index_of_wild_card_being_played {
                    let mut wild_card = current_player.hand.remove(wild_index as usize);
                    match input_text.to_lowercase().as_str() {
                        "r" => wild_card.color = Some(Color::Red),
                        "b" => wild_card.color = Some(Color::Blue),
                        "g" => wild_card.color = Some(Color::Green),
                        "y" => wild_card.color = Some(Color::Yellow),
                        _ => return,
                    }
                    played_card = Some(wild_card);
                    self.index_of_wild_card_being_played = None;
                } else if input_text.to_lowercase().as_str() == "d" {
                    // Draw a card
                    draw_cards(
                        &mut current_player.hand,
                        1,
                        &mut self.deck,
                        &mut self.discard,
                    );
                }
            }
            Input::Number(card_index) => {
                // Player is trying to play a card, we validate it.
                let current_player = &mut self.players[self.current_player_index as usize];
                let card_to_play = current_player.hand.get((card_index - 1) as usize);
                if card_to_play.is_none() {
                    println!("You do not have that card, please pick another.");
                    return;
                }
                let card_to_play = card_to_play.unwrap();

                let last_played_card = self.discard.last();
                if !can_play_card(last_played_card, &card_to_play) {
                    println!("Can't play that card :( , pick another.");
                    return;
                }

                if card_to_play.wild {
                    self.index_of_wild_card_being_played = Some(card_index - 1);
                    return;
                }

                played_card = Some(current_player.hand.remove((card_index - 1) as usize));
            }
        }

        if let Some(played_card) = played_card {
            let next_player_index = get_next_player_index(
                self.current_player_index,
                self.players.len() as i32,
                self.turn_order,
                played_card.turn_effect,
            );

            if let Some(draw_effect) = played_card.draw_effect {
                match draw_effect {
                    DrawEffect::Draw(num_cards_to_draw) => {
                        let next_player = &mut self.players[next_player_index as usize];
                        draw_cards(
                            &mut next_player.hand,
                            num_cards_to_draw,
                            &mut self.deck,
                            &mut self.discard,
                        );
                        println!(
                            "Player {} drew {} cards!",
                            next_player_index + 1,
                            num_cards_to_draw
                        );
                    }
                }
            }

            if let Some(turn_effect) = played_card.turn_effect {
                if turn_effect == TurnEffect::Reverse {
                    match self.turn_order {
                        TurnOrder::Forward => self.turn_order = TurnOrder::Backward,
                        TurnOrder::Backward => self.turn_order = TurnOrder::Forward,
                    }
                }
            }

            self.discard.push(played_card);

            let current_player = &mut self.players[self.current_player_index as usize];
            if current_player.hand.len() == 1 {
                println!("Player {} has uno!", self.current_player_index);
            } else if current_player.hand.len() == 0 {
                println!("Player {} won!", self.current_player_index);
            }

            if !self.game_over() {
                self.current_player_index = next_player_index;
            }
        }
    }

    pub fn render(&self) {
        let current_player = &self.players[self.current_player_index as usize];

        if self.index_of_wild_card_being_played.is_some() {
            println!("What color do you want your wild card to be?");
            println!("Enter one of \"R\", \"B\", \"G\", or \"Y\" to pick a color: ");
            return;
        }

        // TODO: handle ai players
        // "Player 3 is a computer, press Enter to watch their turn.
        // only print: "AI Player 4 drew 3 cards and played a Red 5"
        // or "AI Player 3 played a Red 5"
        // self.automate_player_turn(&mut player) -> String (what happened)
        // Maybe the above is a reason to introduce another step besides input & render
        // Could be 1) input 2) update 3) render
        // Where during update we play out an ai's turn
        println!();
        println!("It is player {}'s turn.", 1 + self.current_player_index);

        println!("Here are your cards:");
        for (i, card) in current_player.hand.iter().enumerate() {
            println!("{}) {}", 1 + i, card);
        }
        println!();

        if let Some(last_played_card) = &self.discard.last() {
            println!("The last played card was {}", last_played_card);
        }

        if self.discard.len() == 0 {
            println!("Type a number to play the first card: ")
        } else {
            println!("Type a number to play a card, or \"d\" to draw a card: ")
        }
    }

    pub fn game_over(&self) -> bool {
        self.players.iter().any(|player| player.hand.len() == 0)
    }
}

fn get_next_player_index(
    current_player_index: i32,
    num_players: i32,
    turn_order: TurnOrder,
    turn_effect: Option<TurnEffect>,
) -> i32 {
    let change_magnitude = if Some(TurnEffect::Skip) == turn_effect {
        2
    } else {
        1
    };
    let change_direction = {
        let initial_direction = match turn_order {
            TurnOrder::Forward => 1,
            TurnOrder::Backward => -1,
        };
        if let Some(TurnEffect::Reverse) = turn_effect {
            initial_direction * -1
        } else {
            initial_direction
        }
    };
    let mut next_player_index = current_player_index;
    for _ in 0..change_magnitude {
        next_player_index += change_direction;
        if next_player_index == num_players {
            next_player_index = 0;
        } else if next_player_index < 0 {
            next_player_index = num_players - 1;
        }
    }
    next_player_index
}

#[derive(Copy, Clone)]
enum TurnOrder {
    Forward,
    Backward,
}

impl Default for Uno {
    fn default() -> Self {
        Uno::new(2)
    }
}

pub fn can_play_card(prev_card: Option<&Card>, next_card: &Card) -> bool {
    if prev_card.is_none() {
        return true;
    }
    let prev_card = prev_card.unwrap();
    if next_card.wild {
        return true;
    }
    // If same color
    if let (Some(next_card_color), Some(prev_card_color)) = (next_card.color, prev_card.color) {
        if next_card_color == prev_card_color {
            return true;
        }
    }
    // If same number
    if let (Some(next_card_number), Some(prev_card_number)) = (next_card.number, prev_card.number) {
        if next_card_number == prev_card_number {
            return true;
        }
    }
    // If same turn effect
    if let (Some(next_card_turn_effect), Some(prev_card_turn_effect)) =
        (next_card.turn_effect, prev_card.turn_effect)
    {
        if next_card_turn_effect == prev_card_turn_effect {
            return true;
        }
    }
    // If both cause drawing cards
    if next_card.draw_effect.is_some() && prev_card.draw_effect.is_some() {
        return true;
    }
    return false;
}

pub fn create_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    // There are 108 cards in a standard Uno deck.
    for i in 0..108 {
        deck.push({
            let mut new_card = Card::default();
            if i < 76 {
                // Cards 1-76: 76 colored & numbered cards. There are 19 of each color.
                // They are numbered 0-9, each color has one 0 and two of 1-9.
                new_card.number = Some({
                    let number = i % 19;
                    let number = (number as f32 / 2.0).ceil();
                    number as i32
                });
                new_card.color = Some({
                    if i / 19 < 1 {
                        Color::Blue
                    } else if i / 19 < 2 {
                        Color::Red
                    } else if i / 19 < 3 {
                        Color::Yellow
                    } else {
                        Color::Green
                    }
                });
            } else if i < 84 {
                // Cards 77-84: 8 colored skip cards
                new_card.turn_effect = Some(TurnEffect::Skip);
                new_card.color = Some({
                    if i - 76 < 2 {
                        Color::Blue
                    } else if i - 76 < 4 {
                        Color::Red
                    } else if i - 76 < 6 {
                        Color::Yellow
                    } else {
                        Color::Green
                    }
                })
            } else if i < 92 {
                // Cards 85-92: 8 colored reverse cards
                new_card.turn_effect = Some(TurnEffect::Reverse);
                new_card.color = Some({
                    if i - 84 < 2 {
                        Color::Blue
                    } else if i - 84 < 4 {
                        Color::Red
                    } else if i - 84 < 6 {
                        Color::Yellow
                    } else {
                        Color::Green
                    }
                })
            } else if i < 100 {
                // Cards 93-100: 8 colored draw 2 cards
                new_card.draw_effect = Some(DrawEffect::Draw(2));
                new_card.color = Some({
                    if i - 92 < 2 {
                        Color::Blue
                    } else if i - 92 < 4 {
                        Color::Red
                    } else if i - 92 < 6 {
                        Color::Yellow
                    } else {
                        Color::Green
                    }
                })
            } else if i < 104 {
                // Cards 101-104: 4 wild cards
                new_card.wild = true;
            } else if i < 108 {
                // Cards 105-108: 4 wild draw 4 cards
                new_card.wild = true;
                new_card.draw_effect = Some(DrawEffect::Draw(4));
            };
            new_card
        });
    }
    return deck;
}

fn draw_cards(
    player_hand: &mut Vec<Card>,
    num_to_draw: i32,
    deck: &mut Vec<Card>,
    discard: &mut Vec<Card>,
) {
    if num_to_draw <= 0 {
        return;
    }

    // -1 because you have to leave the top card in the discard pile and cant put it in the deck
    if deck.len() + discard.len() - 1 < num_to_draw as usize {
        panic!(
            "There are not enough cards left for the player to draw. {} cards in deck + discard but player wants to draw {}.",
            deck.len() + discard.len(),
            num_to_draw
        );
    }

    if deck.len() < num_to_draw as usize {
        for _ in 0..discard.len() {
            deck.push(discard.pop().unwrap())
        }
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
    }

    for _ in 0..num_to_draw {
        player_hand.push(deck.pop().unwrap());
    }
}

#[derive(Default)]
pub struct Player {
    hand: Vec<Card>,
}

#[cfg(test)]
mod tests {
    mod can_play_card {
        use crate::{
            card::{Card, Color, DrawEffect, TurnEffect},
            game::can_play_card,
        };

        #[test]
        fn can_play_same_colors() {
            let mut prev_card = Card::default();
            prev_card.color = Some(Color::Blue);
            let mut next_card = Card::default();
            next_card.color = Some(Color::Blue);
            assert!(can_play_card(Some(&prev_card), &next_card));
        }

        #[test]
        fn cant_play_different_colors() {
            let mut prev_card = Card::default();
            prev_card.color = Some(Color::Blue);
            let mut next_card = Card::default();
            next_card.color = Some(Color::Red);
            assert!(!can_play_card(Some(&prev_card), &next_card));
        }

        #[test]
        fn can_play_same_numbers() {
            let mut prev_card = Card::default();
            prev_card.number = Some(5);
            let mut next_card = Card::default();
            next_card.number = Some(5);
            assert!(can_play_card(Some(&prev_card), &next_card));
        }

        #[test]
        fn cant_play_different_numbers() {
            let mut prev_card = Card::default();
            prev_card.number = Some(5);
            let mut next_card = Card::default();
            next_card.number = Some(2);
            assert!(!can_play_card(Some(&prev_card), &next_card));
        }

        #[test]
        fn can_play_same_turn_effects() {
            let mut prev_card = Card::default();
            prev_card.turn_effect = Some(TurnEffect::Skip);
            let mut next_card = Card::default();
            next_card.turn_effect = Some(TurnEffect::Skip);
            assert!(can_play_card(Some(&prev_card), &next_card));
        }

        #[test]
        fn cant_play_different_turn_effects() {
            let mut prev_card = Card::default();
            prev_card.turn_effect = Some(TurnEffect::Skip);
            let mut next_card = Card::default();
            next_card.turn_effect = Some(TurnEffect::Reverse);
            assert!(!can_play_card(Some(&prev_card), &next_card));
        }

        #[test]
        fn can_play_same_draw_effects() {
            let mut prev_card = Card::default();
            prev_card.draw_effect = Some(DrawEffect::Draw(2));
            let mut next_card = Card::default();
            next_card.draw_effect = Some(DrawEffect::Draw(2));
            assert!(can_play_card(Some(&prev_card), &next_card));
        }

        #[test]
        fn can_play_wild_on_any_color() {
            let mut prev_card = Card::default();
            prev_card.color = Some(Color::Red);
            let mut next_card = Card::default();
            next_card.wild = true;
            assert!(can_play_card(Some(&prev_card), &next_card));
        }

        #[test]
        fn can_play_wild_on_any_number() {
            let mut prev_card = Card::default();
            prev_card.number = Some(8);
            let mut next_card = Card::default();
            next_card.wild = true;
            assert!(can_play_card(Some(&prev_card), &next_card));
        }
    }

    mod create_deck {
        use crate::{card::Card, game::create_deck};

        #[test]
        fn contains_all_standard_cards() {
            fn create_cards_from_strs(strs: Vec<&str>) -> Vec<Card> {
                strs.iter().map(|str| Card::from(str.clone())).collect()
            }

            let standard_cards: Vec<Card> = create_cards_from_strs(vec![
                "blue 0",
                "blue 1",
                "blue 1",
                "blue 2",
                "blue 2",
                "blue 3",
                "blue 3",
                "blue 4",
                "blue 4",
                "blue 5",
                "blue 5",
                "blue 6",
                "blue 6",
                "blue 7",
                "blue 7",
                "blue 8",
                "blue 8",
                "blue 9",
                "blue 9",
                "red 1",
                "red 1",
                "red 2",
                "red 2",
                "red 3",
                "red 3",
                "red 4",
                "red 4",
                "red 5",
                "red 5",
                "red 6",
                "red 6",
                "red 7",
                "red 7",
                "red 8",
                "red 8",
                "red 9",
                "red 9",
                "green 0",
                "green 1",
                "green 1",
                "green 2",
                "green 2",
                "green 3",
                "green 3",
                "green 4",
                "green 4",
                "green 5",
                "green 5",
                "green 6",
                "green 6",
                "green 7",
                "green 7",
                "green 8",
                "green 8",
                "green 9",
                "green 9",
                "yellow 0",
                "yellow 1",
                "yellow 1",
                "yellow 2",
                "yellow 2",
                "yellow 3",
                "yellow 3",
                "yellow 4",
                "yellow 4",
                "yellow 5",
                "yellow 5",
                "yellow 6",
                "yellow 6",
                "yellow 7",
                "yellow 7",
                "yellow 8",
                "yellow 8",
                "yellow 9",
                "yellow 9",
                "blue skip",
                "blue skip",
                "red skip",
                "red skip",
                "green skip",
                "green skip",
                "yellow skip",
                "yellow skip",
                "blue reverse",
                "blue reverse",
                "red reverse",
                "red reverse",
                "green reverse",
                "green reverse",
                "yellow reverse",
                "yellow reverse",
                "blue draw2",
                "blue draw2",
                "red draw2",
                "red draw2",
                "green draw2",
                "green draw2",
                "yellow draw2",
                "yellow draw2",
                "wild",
                "wild",
                "wild",
                "wild",
                "wild draw4",
                "wild draw4",
                "wild draw4",
                "wild draw4",
            ]);
            fn remove_card_or_panic(card_to_remove: &Card, cards: &mut Vec<Card>) {
                let found_card_position =
                    cards.iter().position(|card: &Card| card == card_to_remove);
                if found_card_position.is_none() {
                    println!("Could not find card: {:?}", card_to_remove);
                }
                cards.remove(found_card_position.unwrap());
            }

            let mut received_cards = create_deck();

            for card in standard_cards {
                remove_card_or_panic(&card, &mut received_cards);
            }
        }
    }

    mod get_next_player_index {
        use super::super::*;

        #[test]
        fn after_normal_turn() {
            let result = get_next_player_index(0, 2, TurnOrder::Forward, None);
            assert_eq!(result, 1);
        }

        #[test]
        fn after_reverse() {
            let result = get_next_player_index(0, 3, TurnOrder::Forward, Some(TurnEffect::Reverse));
            assert_eq!(result, 2)
        }

        #[test]
        fn after_skip() {
            let result = get_next_player_index(0, 3, TurnOrder::Forward, Some(TurnEffect::Skip));
            assert_eq!(result, 2);
        }

        #[test]
        fn after_skip_last_player() {
            let result = get_next_player_index(2, 3, TurnOrder::Forward, Some(TurnEffect::Skip));
            assert_eq!(result, 1);
        }
    }

    mod draw_cards {
        use super::super::*;

        #[test]
        fn deals_cards_into_players_hand() {
            let mut hand: Vec<Card> = Vec::new();
            let mut deck = vec![Card::default(), Card::default(), Card::default()];
            let mut discard: Vec<Card> = Vec::new();
            draw_cards(&mut hand, 2, &mut deck, &mut discard);
            assert!(hand.len() == 2);
            assert!(deck.len() == 1);
        }

        #[test]
        fn moves_cards_from_discard_into_deck_if_deck_doesnt_have_enough() {
            let mut hand: Vec<Card> = Vec::new();
            let mut deck: Vec<Card> = Vec::new();
            let mut discard = vec![Card::default(), Card::default(), Card::default()];
            draw_cards(&mut hand, 2, &mut deck, &mut discard);
            assert!(hand.len() == 2);
            assert!(deck.len() == 1);
            assert!(discard.len() == 0);
        }

        #[test]
        #[should_panic]
        fn panics_if_there_arent_enough_cards_in_deck_and_discard_for_player_to_draw() {
            let mut hand: Vec<Card> = Vec::new();
            let mut deck = vec![Card::default()];
            let mut discard = vec![Card::default()];
            draw_cards(&mut hand, 3, &mut deck, &mut discard);
        }
    }
}
