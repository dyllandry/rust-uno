use crate::card::Card;

#[derive(Default)]
pub struct UI {
    pub last_turn_recap: Option<TurnRecap>,
    pub player_instruction: Option<PlayerInstruction>,
    pub uno_declarations: Vec<i32>,
    pub error: Option<String>,
    pub winning_player: Option<i32>,
    displayed_hand: Option<DisplayedHand>,
}

impl UI {
    pub fn render(&self) {
        UI::clear_screen();

        if let Some(winning_player) = self.winning_player {
            println!("Player {} won!", winning_player);
            return;
        }

        if let Some(last_turn_recap) = &self.last_turn_recap {
            if last_turn_recap.drawn_cards > 0 {
                println!("Player {} drew {} cards!", last_turn_recap.player, last_turn_recap.drawn_cards);
            }
            for played_card in &last_turn_recap.played_cards {
                println!("Player {} played a {}!", last_turn_recap.player, played_card);
            }
            println!();
        }

        if self.uno_declarations.len() > 0 {
            for player in &self.uno_declarations {
                println!("Player {} has uno!", player);
            }
            println!();
        }

        if let Some(displayed_hand) = &self.displayed_hand {
            println!("Player {}'s cards:", displayed_hand.player);
            for (i, card) in displayed_hand.cards.iter().enumerate() {
                println!("{}) {}", 1 + i, card);
            }
            println!();
        }

        if let Some(player_instruction) = &self.player_instruction {
            match player_instruction {
                PlayerInstruction::PickCard => {
                    println!("Type a number to play a card, or \"d\" to draw a card: ")
                },
                PlayerInstruction::PickWildColor => {
                    println!("What color do you want your wild card to be?");
                    println!("Enter one of \"R\", \"B\", \"G\", or \"Y\" to pick a color: ");
                },
            }
        }

        if let Some(error) = &self.error {
            println!();
            println!("{}", error);
        }

    }

    pub fn clear_screen() {
        // ANSI escape codes: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
        print!("\x1B[2J");
        print!("\x1B[H");
    }

    pub fn display_hand(&mut self, player: i32, cards: &Vec<Card>) {
        self.displayed_hand = Some(
            DisplayedHand {
                player,
                cards: cards.clone(),
            }
        )
    }
}

pub struct TurnRecap {
    pub player: i32,
    pub played_cards: Vec<Card>,
    pub drawn_cards: i32,
}

pub enum PlayerInstruction {
    PickCard,
    PickWildColor
}

pub struct DisplayedHand {
    pub player: i32,
    pub cards: Vec<Card>
}
