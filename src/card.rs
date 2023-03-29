use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Red => "red".to_string(),
            Color::Blue => "blue".to_string(),
            Color::Green => "green".to_string(),
            Color::Yellow => "yellow".to_string(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TurnEffect {
    Skip,
    Reverse,
}

impl ToString for TurnEffect {
    fn to_string(&self) -> String {
        match self {
            TurnEffect::Skip => "skip".to_string(),
            TurnEffect::Reverse => "reverse".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DrawEffect {
    Draw(i32),
}

impl ToString for DrawEffect {
    fn to_string(&self) -> String {
        match self {
            DrawEffect::Draw(num_cards) => format!("draw {}", num_cards),
        }
    }
}

#[derive(Default, Debug)]
pub struct Card {
    pub number: Option<i32>,
    pub color: Option<Color>,
    pub turn_effect: Option<TurnEffect>,
    pub draw_effect: Option<DrawEffect>,
    pub wild: bool,
}

impl Card {
    fn render(&self) -> String {
        let mut description_parts: Vec<String> = Vec::new();
        if self.wild {
            description_parts.push("wild".to_string());
        }
        if let Some(color) = self.color {
            description_parts.push(color.to_string());
        }
        if let Some(number) = self.number {
            description_parts.push(number.to_string());
        }
        if let Some(turn_effect) = self.turn_effect {
            description_parts.push(turn_effect.to_string());
        }
        if let Some(draw_effect) = self.draw_effect {
            description_parts.push(draw_effect.to_string());
        }
        description_parts.join(" ")
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// Used to help make lots of cards with not a lot of typing.
/// Can take something like "red 5" and give back a card instance that's a red 5.
/// Or you can do "green reverse" to get a green reverse card.
/// Only catch is for draw effects only "draw2" and "draw4" are supported.
/// So to get a red draw2 you'd have to do Card::from("red draw2");
impl From<&str> for Card {
    fn from(card_string: &str) -> Self {
        let mut card = Card::default();
        for card_descriptor in card_string.split(' ') {
            match card_descriptor {
                "red" => card.color = Some(Color::Red),
                "blue" => card.color = Some(Color::Blue),
                "yellow" => card.color = Some(Color::Yellow),
                "green" => card.color = Some(Color::Green),
                "0" => card.number = Some(0),
                "1" => card.number = Some(1),
                "2" => card.number = Some(2),
                "3" => card.number = Some(3),
                "4" => card.number = Some(4),
                "5" => card.number = Some(5),
                "6" => card.number = Some(6),
                "7" => card.number = Some(7),
                "8" => card.number = Some(8),
                "9" => card.number = Some(9),
                "skip" => card.turn_effect = Some(TurnEffect::Skip),
                "reverse" => card.turn_effect = Some(TurnEffect::Reverse),
                "draw2" => card.draw_effect = Some(DrawEffect::Draw(2)),
                "draw4" => card.draw_effect = Some(DrawEffect::Draw(4)),
                "wild" => card.wild = true,
                _ => (),
            };
        }
        card
    }
}

impl Eq for Card {}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        // check color
        if self.color.is_some() != other.color.is_some() {
            return false;
        }
        if let (Some(self_color), Some(other_color)) = (self.color, other.color) {
            if self_color != other_color {
                return false;
            }
        }
        // check number
        if self.number.is_some() != other.number.is_some() {
            return false;
        }
        if let (Some(self_number), Some(other_number)) = (self.number, other.number) {
            if self_number != other_number {
                return false;
            }
        }
        // check turn effect
        if self.turn_effect.is_some() != other.turn_effect.is_some() {
            return false;
        }
        if let (Some(self_turn_effect), Some(other_turn_effect)) =
            (self.turn_effect, other.turn_effect)
        {
            if self_turn_effect != other_turn_effect {
                return false;
            }
        }
        // check draw effect
        if self.draw_effect.is_some() != other.draw_effect.is_some() {
            return false;
        }
        if let (Some(self_draw_effect), Some(other_draw_effect)) =
            (self.draw_effect, other.draw_effect)
        {
            if self_draw_effect != other_draw_effect {
                return false;
            }
        }
        // check wild
        if self.wild != other.wild {
            return false;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    mod card_equality {
        use crate::card::{self, Card, Color, DrawEffect};

        #[test]
        fn red_1_equals_red_1() {
            let mut red_1_1 = Card::default();
            red_1_1.color = Some(Color::Red);
            red_1_1.number = Some(1);
            let mut red_1_2 = Card::default();
            red_1_2.color = Some(Color::Red);
            red_1_2.number = Some(1);
            assert!(red_1_1 == red_1_2);
        }

        #[test]
        fn blue_skip_equals_blue_skip() {
            let mut blue_skip_1 = Card::default();
            blue_skip_1.color = Some(Color::Blue);
            blue_skip_1.turn_effect = Some(crate::card::TurnEffect::Skip);
            let mut blue_skip_2 = Card::default();
            blue_skip_2.color = Some(Color::Blue);
            blue_skip_2.turn_effect = Some(crate::card::TurnEffect::Skip);
            assert!(blue_skip_1 == blue_skip_2);
        }

        #[test]
        fn red_draw_3_equals_red_draw_3() {
            let mut red_draw_3_1 = Card::default();
            red_draw_3_1.color = Some(Color::Red);
            red_draw_3_1.draw_effect = Some(DrawEffect::Draw(3));
            let mut red_draw_3_2 = Card::default();
            red_draw_3_2.color = Some(Color::Red);
            red_draw_3_2.draw_effect = Some(DrawEffect::Draw(3));
            assert!(red_draw_3_1 == red_draw_3_2);
        }

        #[test]
        fn green_2_doesnt_equal_blue_1() {
            let mut green_2 = Card::default();
            green_2.color = Some(Color::Green);
            green_2.number = Some(2);
            let mut blue_1 = Card::default();
            blue_1.color = Some(Color::Blue);
            blue_1.number = Some(1);
            assert!(green_2 != blue_1);
        }

        #[test]
        fn blue_skip_doesnt_equal_red_draw_2() {
            let mut blue_skip = Card::default();
            blue_skip.color = Some(Color::Blue);
            blue_skip.turn_effect = Some(card::TurnEffect::Skip);
            let mut red_draw_2 = Card::default();
            red_draw_2.color = Some(Color::Red);
            red_draw_2.draw_effect = Some(DrawEffect::Draw(2));
            assert!(blue_skip != red_draw_2);
        }
    }

    mod from_str {
        use crate::card::{Card, Color, DrawEffect};

        #[test]
        fn create_blue_1() {
            let mut blue_1 = Card::default();
            blue_1.color = Some(Color::Blue);
            blue_1.number = Some(1);
            let received_card = Card::from("blue 1");
            assert!(received_card == blue_1);
        }

        #[test]
        fn create_green_draw_4() {
            let mut green_draw_4 = Card::default();
            green_draw_4.color = Some(Color::Green);
            green_draw_4.draw_effect = Some(DrawEffect::Draw(4));
            let received_card = Card::from("green draw4");
            assert!(green_draw_4 == received_card);
        }

        #[test]
        fn create_wild_yellow() {
            let mut wild_yellow = Card::default();
            wild_yellow.color = Some(Color::Yellow);
            wild_yellow.wild = true;
            let received_card = Card::from("wild yellow");
            assert!(wild_yellow == received_card);
        }
    }
}
