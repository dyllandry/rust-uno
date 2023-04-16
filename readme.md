# Todo

- handle no cards being left when a card is drawn
```rust
/// Panics if the number of cards to draw is less than the amount of cards in
/// the deck and discard pile.
fn draw_cards(&mut player_hand, num_to_draw, &mut deck, &mut discard) {
	for _ in 0..num_to_draw {
		if deck.len() == 0 {
			// add discard to deck then shuffle
		}
		player_hand.push(deck.pop());
	}
}
```

- AI

# Designing turn logic

## start game

- create & shuffle deck
- deal players in

## turn loop

(If deck ever runs out of cards, shuffle in the discard pile)

- next player plays a card
- if they can't, they draw a card
- wild cards need the player to pick a color
- draw effect cards can the next player draw cards
- turn effect cards can affect who goes next
- If the player hits 1 card left should announce it
- When a player plays their last card they win

# Input

# Rendering

1. show last played card
1. list any players that have uno
1. show current player # and cards
