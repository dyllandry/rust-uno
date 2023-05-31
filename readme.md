# Bugs

- there might be some display bugs since after an input I assume the next render will render that last player's move, but that's not always true. All upcoming ai players before the next human player will go after the last human player's input, but before the next render. That means before each render it's not the human that last went, but the ai. This can cause display bugs where I assume that the render is immediately after a human player's move.
- There's still a weird bug where the AI can play cards that do not match the last played card
	- OH I think it's when the ai plays two skips in a row of different colors then a third card
		- P1: red 5
		- P2: red skip
		- P2: blue skip
		- P2: blue 3
	- maybe something happens weird when the player is drawing cards and the discard is shuffled into the deck
		- because on one of the occurences, it was the 3rd time a card was played when there's only 2 in the deck
		- Maybe some sort of index isn't being updated or managed right. Like after drawing some cards and shuffling the deck.
		- Maybe there's something in the automate_turn code that isn't validating a card right in some cases. Or playing the wrong one.
			- Like it valides a card, goes to play it, but then picks the wrong card somehow
- wild cards don't get color reset after deck gets reshuffled

# Todo

# Someday

- Would be cool to also show the list of cards in the deck and discard pile for debugging purposes
- Would be cool to create ui "boxes" that I could display text in that wrapped if the displayed text was too long.
	- that way I could have vertical sections of the screen dedicated to different things. Far left could be the current hand, middle could be discard, idk.
	- Or I could just use an actual TUI library at that point
