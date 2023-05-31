use rand::{seq::SliceRandom, thread_rng};

use crate::{
    card::{Card, Color, DrawEffect, TurnEffect},
    user_input::Input, ui::{UI, DisplayedHand, PlayerInstruction, TurnRecap},
};

/**
 This mod is for game rules.
*/

pub struct Uno {
    players: Vec<Player>,
    current_player_index: i32,
    deck: Vec<Card>,
    discard: Vec<Card>,
    wild_card_index_to_pick_color_for: Option<i32>,
    turn_order: TurnOrder,
    ui: UI,
}

impl Uno {
    pub fn new(player_count: i32, ai_count: i32) -> Self {
        if player_count == 0 {
            panic!("Only Uno games with at least 1 human player are supported.");
        }

        let mut game = Uno {
            current_player_index: 0,
            players: Vec::new(),
            deck: Vec::new(),
            discard: Vec::new(),
            wild_card_index_to_pick_color_for: None,
            turn_order: TurnOrder::Forward,
            ui: UI::default()
        };

        game.deck = create_deck();
        game.deck.shuffle(&mut thread_rng());

        let mut human_players: Vec<Player> = Vec::new();
        for _ in 0..player_count {
            let mut player = Player::default();
            draw_cards(&mut player.hand, 7, &mut game.deck, &mut game.discard);
            human_players.push(player);
        }

        let mut ai_players: Vec<Player> = Vec::new();
        for _ in 0..ai_count {
            let mut player = Player::default();
            player.ai = true;
            draw_cards(&mut player.hand, 7, &mut game.deck, &mut game.discard);
            ai_players.push(player);
        }

        // Alternate order of players and ai so players play against ai.
        for _ in 0..(player_count + ai_count) {
            let human: Option<Player> = human_players.pop();
            let ai: Option<Player> = ai_players.pop();
            if let Some(human) = human {
                game.players.push(human);
            }
            if let Some(ai) = ai {
                game.players.push(ai);
            }
        }

        game.ui.display_hand(1, &game.players.first().unwrap().hand);
        game.ui.player_instruction = Some(PlayerInstruction::PickCard);

        game
    }

    pub fn input(&mut self, input: Input) {
        // Create a valid game command from raw user input.
        let command: Option<Command> = match input {
            Input::Number(card_index) => {
                Some(Command::PickCardToPlay(card_index))
            }
            Input::Text(input_text) => {
                if input_text.to_lowercase().as_str() == "d" {
                    Some(Command::DrawCard)
                } else if self.wild_card_index_to_pick_color_for.is_some() {
                    let picked_wild_color: Option<Color> = match input_text.to_lowercase().as_str() {
                        "r" => Some(Color::Red),
                        "b" => Some(Color::Blue),
                        "g" => Some(Color::Green),
                        "y" => Some(Color::Yellow),
                        _ => None,
                    };
                    picked_wild_color.and_then(|color| Some(Command::PickWildCardColor(color)))
                } else {
                    None
                }
            }
        };

        // Process command
        if let Some(command) = command {
            match command {
                Command::DrawCard => {
                    let current_player = &mut self.players[self.current_player_index as usize];
                    draw_cards(
                        &mut current_player.hand,
                        1,
                        &mut self.deck,
                        &mut self.discard,
                    );
                    self.ui.display_hand(self.current_player_index + 1, &current_player.hand);
                },
                Command::PickWildCardColor(wild_color) => {
                    let current_player = &mut self.players[self.current_player_index as usize];
                    if let Some(wild_index) = self.wild_card_index_to_pick_color_for {
                        let mut wild_card = current_player.hand.remove((wild_index - 1) as usize);
                        wild_card.color = Some(wild_color);
                        self.ui.player_instruction = None;
                        self.wild_card_index_to_pick_color_for = None;
                        self.play_card(wild_card);
                    };
                },
                Command::PickCardToPlay(card_index) => {
                    self.wild_card_index_to_pick_color_for = None;
                    let current_player = &mut self.players[self.current_player_index as usize];
                    match validate_card_from_index(card_index, &current_player.hand, self.discard.last()) {
                        CardFromIndexValidationResult::Invalid(reason) => {
                            self.ui.error = Some(reason);
                        },
                        CardFromIndexValidationResult::Valid => {
                            let card_to_play = current_player.hand.get((card_index - 1) as usize).unwrap();
                            // If the picked a wild card to play, then they next need to pick a color. We
                            // wait for an upcoming Command::PickWildColor(color)
                            if card_to_play.wild {
                                self.ui.player_instruction = Some(PlayerInstruction::PickWildColor);
                                self.wild_card_index_to_pick_color_for = Some(card_index);
                            } else {
                                let card_to_play = current_player.hand.remove((card_index - 1) as usize);
                                self.ui.last_turn_recap = Some(TurnRecap {
                                    player: self.current_player_index + 1,
                                    card: card_to_play,
                                    drawn_cards: 0,
                                });
                                self.ui.error = None;
                                self.play_card(card_to_play);
                            }
                        }
                    }
                },
            }

            // Let AI players go
            if !self.game_over() {
                let mut current_player = self.players.get(self.current_player_index as usize).unwrap();
                if current_player.ai {
                    while !self.game_over() && current_player.ai {
                        self.automate_current_player_turn();
                        current_player = self.players.get(self.current_player_index as usize).unwrap();
                    }
                    self.ui.player_instruction = Some(PlayerInstruction::PickCard);
                }
                self.ui.display_hand(self.current_player_index + 1, &current_player.hand);

                let players_with_uno = self.players.iter().enumerate().filter_map(|(index, player)| {
                    if player.hand.len() == 1 {
                        return Some((index + 1) as i32);
                    } else {
                        None
                    }
                }).collect();

                self.ui.uno_declarations = players_with_uno;
            }

            if self.game_over() {
                let winning_player = self.players.iter().enumerate().find_map(|(index, player)| {
                    if player.hand.len() == 0 {
                        return Some((index + 1) as i32);
                    } else {
                        return None;
                    }
                }).unwrap();
                self.ui.winning_player = Some(winning_player);
            }
        }
    }

    /// Card validation should be done prior to calling this function.
    fn play_card(&mut self, card: Card) {
        if card.turn_effect == Some(TurnEffect::Reverse) {
            match self.turn_order {
                TurnOrder::Forward => self.turn_order = TurnOrder::Backward,
                TurnOrder::Backward => self.turn_order = TurnOrder::Forward,
            }
        }

        let next_player_index = {
            let mut next_player_index = get_next_player_index(
                self.current_player_index,
                self.players.len() as i32,
                self.turn_order,
            );

            if card.turn_effect == Some(TurnEffect::Skip) {
                next_player_index = get_next_player_index(
                    next_player_index,
                    self.players.len() as i32,
                    self.turn_order,
                );
            }
            next_player_index
        };

        if let Some(draw_effect) = card.draw_effect {
            play_card_draw_effect(
                &draw_effect,
                &mut self.players[next_player_index as usize],
                next_player_index,
                &mut self.deck,
                &mut self.discard
            );
        }

        self.discard.push(card);

        // Set next player for next turn
        if !self.game_over() {
            self.current_player_index = next_player_index;
        }
    }

    fn automate_current_player_turn(&mut self) {
        let player = &mut self.players[self.current_player_index as usize];
        let mut num_drawn_cards = 0;
        let mut card_index_to_play: Option<usize> = None;
        while card_index_to_play.is_none() {
            let last_played_card: Option<&Card> = self.discard.last();
            if let Some((card_index, _)) = player.hand.iter().enumerate().find(|(_, card)| can_play_card(last_played_card, card)) {
                card_index_to_play = Some(card_index);
            } else {
                num_drawn_cards += 1;
                draw_cards(
                    &mut player.hand,
                    1,
                    &mut self.deck,
                    &mut self.discard,
                );
            }
        }
        let mut card_to_play = player.hand.remove(card_index_to_play.unwrap());
        if card_to_play.wild {
            let mut color_counters: Vec<(Color, i32)> = vec![
                {(Color::Red, 0)},
                {(Color::Blue, 0)},
                {(Color::Yellow, 0)},
                {(Color::Green, 0)},
            ];
            for counter in &mut color_counters {
                counter.1 = player.hand.iter().filter(|card| card.color == Some(counter.0)).count() as i32;
            }
            color_counters.sort_by_key(|(_, count)| *count);
            let color_with_most_cards = color_counters.first().unwrap().0;
            card_to_play.color = Some(color_with_most_cards);
        }

        self.ui.last_turn_recap = Some(TurnRecap {
            player: self.current_player_index + 1,
            card: card_to_play,
            drawn_cards: num_drawn_cards,
        });

        self.play_card(card_to_play);
    }

    pub fn render(&self) {
        self.ui.render();
    }

    pub fn game_over(&self) -> bool {
        self.players.iter().any(|player| player.hand.len() == 0)
    }
}

fn play_card_draw_effect(
    draw_effect: &DrawEffect,
    next_player: &mut Player,
    next_player_index: i32,
    deck: &mut Vec<Card>,
    discard: &mut Vec<Card>
) {
    let DrawEffect::Draw(num_cards_to_draw) = draw_effect;
    draw_cards(
        &mut next_player.hand,
        *num_cards_to_draw,
        deck,
        discard,
    );
}

enum CardFromIndexValidationResult {
    Valid,
    Invalid(String),
}

fn validate_card_from_index(card_index: i32, player_hand: &Vec<Card>, last_played_card: Option<&Card>) -> CardFromIndexValidationResult {
    let card_to_play = player_hand.get((card_index - 1) as usize);
    if card_to_play.is_none() {
        return CardFromIndexValidationResult::Invalid("You do not have that card, please pick another.".to_string());
    }

    let card_to_play = card_to_play.unwrap();

    if !can_play_card(last_played_card, &card_to_play) {
        return CardFromIndexValidationResult::Invalid("Can't play that card :( , pick another.".to_string());
    }
    return CardFromIndexValidationResult::Valid
}

fn get_next_player_index(
    current_player_index: i32,
    num_players: i32,
    turn_order: TurnOrder,
) -> i32 {
    let change_direction = match turn_order {
        TurnOrder::Forward => 1,
        TurnOrder::Backward => -1,
    };
    let mut next_player_index = current_player_index + change_direction;
    if next_player_index == num_players {
        next_player_index = 0;
    } else if next_player_index < 0 {
        next_player_index = num_players - 1;
    }
    next_player_index
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum TurnOrder {
    Forward,
    Backward,
}

impl Default for Uno {
    fn default() -> Self {
        Uno::new(1, 1)
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
            let mut card = discard.pop().unwrap();
            if card.wild && card.color.is_some() {
                card.color = None;
            }
            deck.push(card);
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
    ai: bool
}

enum Command {
    PickCardToPlay(i32),
    DrawCard,
    PickWildCardColor(Color),
}

#[cfg(test)]
mod tests {
    mod uno_new {
        use super::super::*;

        #[test]
        fn players_have_7_cards_each() {
            let uno = Uno::new(2,2);
            assert_eq!(uno.players[0].hand.len(), 7);
            assert_eq!(uno.players[1].hand.len(), 7);
            assert_eq!(uno.players[2].hand.len(), 7);
            assert_eq!(uno.players[3].hand.len(), 7);
        }

        #[test]
        fn players_alternate_between_human_and_ai() {
            let uno = Uno::new(2,2);
            assert_eq!(uno.players[0].ai, false);
            assert_eq!(uno.players[1].ai, true);
            assert_eq!(uno.players[2].ai, false);
            assert_eq!(uno.players[3].ai, true);
        }
    }

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
        fn going_forwards() {
            let result = get_next_player_index(0, 2, TurnOrder::Forward);
            assert_eq!(result, 1);
        }

        #[test]
        fn going_backwards() {
            let result = get_next_player_index(0, 3, TurnOrder::Backward);
            assert_eq!(result, 2)
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

        #[test]
        fn resets_wild_card_color_when_cards_move_from_discard_to_deck() {
            let mut hand: Vec<Card> = Vec::new();
            let mut deck: Vec<Card> = Vec::new();
            let old_wild_card = Card::from("wild blue");
            let mut discard = vec![old_wild_card];
            draw_cards(&mut hand, 1, &mut deck, &mut discard);
            let drawn_card = hand.get(0).unwrap();
            assert!(drawn_card.color.is_none());
        }
    }

    mod play_card_draw_effect {
        use super::super::*;

        #[test]
        fn two_cards_move_from_the_deck_to_the_next_player_after_a_draw_2_card_is_played() {
            let mut deck: Vec<Card> = vec![
                Card::from("red 1"),
                Card::from("blue 2"),
                Card::from("green 3"),
            ];
            let mut discard: Vec<Card> = vec![];
            let mut next_player = Player::default();
            let draw_effect = DrawEffect::Draw(2);

            play_card_draw_effect(&draw_effect, &mut next_player, 1, &mut deck, &mut discard);

            assert_eq!(deck.len(), 1);
            assert_eq!(next_player.hand.len(), 2);
        }
    }

    mod play_card {
        use super::super::*;

        #[test]
        fn after_player_1_plays_a_skip_player_3_is_next() {
            let mut uno = Uno::new(4, 0);
            let card = Card::from("skip");

            uno.play_card(card);

            assert_eq!(uno.current_player_index, 2);
        }

        #[test]
        fn after_player_1_plays_a_reverse_player_4_is_next() {
            let mut uno = Uno::new(4, 0);
            let card = Card::from("reverse");

            uno.play_card(card);

            assert_eq!(uno.current_player_index, 3);
        }
    }

    mod automate_current_player_turn {
        use super::super::*;

        #[test]
        fn player_plays_valid_card_in_hand() {
            let mut uno = Uno::new(2,0);
            let last_played_card = Card::from("green 5");
            uno.discard.push(last_played_card);
            let bad_card_1 = Card::from("red 1");
            let bad_card_2 = Card::from("yellow 2");
            let valid_card = Card::from("green 3");
            uno.players[0].hand = vec![bad_card_1, valid_card, bad_card_2];

            uno.automate_current_player_turn();
            
            assert_eq!(uno.players[0].hand.len(), 2);
            let played_card = uno.discard.last().unwrap();
            assert_eq!(played_card.color, Some(Color::Green));
            assert_eq!(played_card.number, Some(3));
        }

        #[test]
        fn player_plays_wild_card_in_hand() {
            let mut uno = Uno::new(2,0);
            let wild_card = Card::from("wild");
            uno.players[0].hand = vec![wild_card];

            uno.automate_current_player_turn();
            
            let played_card = uno.discard.last().unwrap();
            assert!(played_card.wild);
            assert!(played_card.color.is_some());
        }

        #[test]
        fn player_draws_when_no_valid_cards_in_hand_and_plays_next_valid_card() {
            let mut uno = Uno::new(2,0);
            let last_played_card = Card::from("green 5");
            uno.discard.push(last_played_card);
            let bad_hand_card_1 = Card::from("red 1");
            let bad_hand_card_2 = Card::from("yellow 2");
            uno.players[0].hand = vec![bad_hand_card_1, bad_hand_card_2];
            let bad_deck_card = Card::from("red 3");
            let valid_deck_card = Card::from("green 4");
            uno.deck = vec![valid_deck_card, bad_deck_card];

            uno.automate_current_player_turn();
            
            assert_eq!(uno.players[0].hand.len(), 3);
            assert_eq!(uno.deck.len(), 0);
            let played_card = uno.discard.last().unwrap();
            assert_eq!(played_card.color, Some(Color::Green));
            assert_eq!(played_card.number, Some(4));
        }
    }

    mod input {
        use super::super::*;

        #[test]
        fn next_ai_player_goes_after_human_provides_input() {
            let mut uno = Uno::new(1,1);
            // Add some cards to the discard pile and player's hands so we can get the human player
            // and then the ai to immediately make valid moves without worrying about whether
            // they've got valid cards in their hands.
            let last_played_card = Card::from("red 1");
            uno.discard.push(last_played_card);
            let valid_next_card = Card::from("red 2");
            let human_player_num_cards_before = {
                let human_player = uno.players.iter_mut().find(|player| !player.ai).unwrap();
                human_player.hand.insert(0, valid_next_card);
                human_player.hand.len()
            };
            let ai_player_num_cards_before = {
                let ai_player = uno.players.iter_mut().find(|player| player.ai).unwrap();
                ai_player.hand.insert(0, valid_next_card);
                ai_player.hand.len()
            };

            uno.input(Input::Number(1));

            let human_player_num_cards_after = {
                let human_player = uno.players.iter_mut().find(|player| !player.ai).unwrap();
                human_player.hand.len()
            };
            let ai_player_num_cards_after = {
                let ai_player = uno.players.iter_mut().find(|player| player.ai).unwrap();
                ai_player.hand.len()
            };

            assert_eq!(human_player_num_cards_before - 1, human_player_num_cards_after);
            assert_eq!(ai_player_num_cards_before - 1, ai_player_num_cards_after);
        }

        #[test]
        fn human_goes_after_ai_player_goes() {
            let mut uno = Uno::new(1,1);
            // Add some cards to the discard pile and player's hands so we can get the human player
            // and then the ai to immediately make valid moves without worrying about whether
            // they've got valid cards in their hands.
            let last_played_card = Card::from("red 1");
            uno.discard.push(last_played_card);
            let valid_next_card = Card::from("red 2");
            {
                let human_player = uno.players.iter_mut().find(|player| !player.ai).unwrap();
                human_player.hand.insert(0, valid_next_card);
            };
            {
                let ai_player = uno.players.iter_mut().find(|player| player.ai).unwrap();
                ai_player.hand.insert(0, valid_next_card);
            };

            // It's technically necessary for a human to go first to start the game, so providing this input gets the human player to go.
            uno.input(Input::Number(1));

            let human_player_num_cards_before = {
                let human_player = uno.players.iter_mut().find(|player| !player.ai).unwrap();
                human_player.hand.insert(0, valid_next_card);
                human_player.hand.len()
            };

            // This input is for the human player's second turn.
            uno.input(Input::Number(1));

            let human_player_num_cards_after = {
                let human_player = uno.players.iter_mut().find(|player| !player.ai).unwrap();
                human_player.hand.len()
            };

            assert_eq!(human_player_num_cards_before - 1, human_player_num_cards_after);
        }
    }
}
