use crate::card::{Card, Color, DrawEffect, TurnEffect};

/**
 This mod is for game rules.
*/

pub fn can_play_card(prev_card: &Card, next_card: &Card) -> bool {
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
            assert!(can_play_card(&prev_card, &next_card));
        }

        #[test]
        fn cant_play_different_colors() {
            let mut prev_card = Card::default();
            prev_card.color = Some(Color::Blue);
            let mut next_card = Card::default();
            next_card.color = Some(Color::Red);
            assert!(!can_play_card(&prev_card, &next_card));
        }

        #[test]
        fn can_play_same_numbers() {
            let mut prev_card = Card::default();
            prev_card.number = Some(5);
            let mut next_card = Card::default();
            next_card.number = Some(5);
            assert!(can_play_card(&prev_card, &next_card));
        }

        #[test]
        fn cant_play_different_numbers() {
            let mut prev_card = Card::default();
            prev_card.number = Some(5);
            let mut next_card = Card::default();
            next_card.number = Some(2);
            assert!(!can_play_card(&prev_card, &next_card));
        }

        #[test]
        fn can_play_same_turn_effects() {
            let mut prev_card = Card::default();
            prev_card.turn_effect = Some(TurnEffect::Skip);
            let mut next_card = Card::default();
            next_card.turn_effect = Some(TurnEffect::Skip);
            assert!(can_play_card(&prev_card, &next_card));
        }

        #[test]
        fn cant_play_different_turn_effects() {
            let mut prev_card = Card::default();
            prev_card.turn_effect = Some(TurnEffect::Skip);
            let mut next_card = Card::default();
            next_card.turn_effect = Some(TurnEffect::Reverse);
            assert!(!can_play_card(&prev_card, &next_card));
        }

        #[test]
        fn can_play_same_draw_effects() {
            let mut prev_card = Card::default();
            prev_card.draw_effect = Some(DrawEffect::Draw(2));
            let mut next_card = Card::default();
            next_card.draw_effect = Some(DrawEffect::Draw(2));
            assert!(can_play_card(&prev_card, &next_card));
        }

        #[test]
        fn can_play_wild_on_any_color() {
            let mut prev_card = Card::default();
            prev_card.color = Some(Color::Red);
            let mut next_card = Card::default();
            next_card.wild = true;
            assert!(can_play_card(&prev_card, &next_card));
        }

        #[test]
        fn can_play_wild_on_any_number() {
            let mut prev_card = Card::default();
            prev_card.number = Some(8);
            let mut next_card = Card::default();
            next_card.wild = true;
            assert!(can_play_card(&prev_card, &next_card));
        }
    }

    mod create_deck {
        use crate::{
            card::Card,
            game::create_deck,
        };

        #[test]
        fn contains_all_standard_cards() {
            fn create_cards_from_strs(strs: Vec<&str>) -> Vec<Card> {
                strs.iter().map(|str| Card::from(str.clone())).collect()
            }

            let standard_cards: Vec<Card> = create_cards_from_strs(
                vec![
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
                ]
            );
            fn remove_card_or_panic(card_to_remove: &Card, cards: &mut Vec<Card>) {
                let found_card_position = cards
                    .iter()
                    .position(|card: &Card| card == card_to_remove);
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
}
