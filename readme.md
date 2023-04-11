# Todo

- try out making turn effect its own function outside of the game struct with dependency injection
	- dont have to setup uno struct to test the function
	- worried it makes the code harder to write or read
- handle no cards being left when a card is drawn
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
