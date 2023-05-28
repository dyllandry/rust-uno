# Todo

## AI

- I should label the last commit as the v1 release since it is working now.

-	change output from stream of text to just display what the player needs to know now
	- Maybe it would be good to stop printing during input, and instead store information to display in a structure that can be printed later in one go. Otherwise the structure of the output is coupled to the order this information is computed.
	- example of datastructure
		```rust
		struct UI {
			players_with_uno: Vec<i32>,
			previous_player_num_drawn_cards: Option<i32>,
			previous_player_last_played_card: Card,
			current_player_cards: Vec<Card>,
			input_error: Option<String>,
		}

		impl UI {
			pub fn render(&self) {}
		}
		```
	- example of output
		```
		(What happened previously)
		[(for each player) Player <player> has uno!]
		[Player <previous player> drew # cards!]
		Player <previous player> played a X

		(Your hand)
		Your cards are:
		1) ...

		(Instruction)
		Type blah to blah.

		(Errors, you can't play X)
		```
	- can use `print!("\x1B[2J")` to clear the screen.
		- it sends an ANSI escape code / control code to the terminal
			- https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
