# Todo

## AI

- get ai players until next human player
- `fn automate_turn(&mut ai.hand, &mut deck, &mut discard)`
	- test: plays valid card in hand
	- test: draws & plays card from deck if no valid card in hand
- there's a lot of stuff that happens in input that would also need to happen in automate_turn like draw effects, turn effects, saying uno, saying someone won. So I think I should organise the input function into smaller pieces. Then I'll be able to reuse code and avoid multiple mutible references as two functions try to do a lot of the same stuff.
	

