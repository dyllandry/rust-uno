use std::fmt::Display;

fn main() {
    let mut card = Card::default();
    card.number = Some(5);
    card.color = Some(Color::Red);
    println!("{}", card);
}

#[derive(Copy, Clone)]
enum Color {
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

#[derive(Copy, Clone)]
enum TurnEffect {
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

#[derive(Copy, Clone)]
enum DrawEffect {
    Draw(i32),
}

impl ToString for DrawEffect {
    fn to_string(&self) -> String {
        match self {
            DrawEffect::Draw(num_cards) => format!("draw {}", num_cards),
        }
    }
}

#[derive(Default)]
struct Card {
    number: Option<i32>,
    color: Option<Color>,
    turn_effect: Option<TurnEffect>,
    draw_effect: Option<DrawEffect>,
    wild: bool,
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
