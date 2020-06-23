# combbreaker

Toy project featuring a Mastermind-like game.

Example game:

```
$ ./clicombbreaker 

Welcome to COMBBREAKER, a game where you have to break a combination by
taking guesses. You will be answered with the number of digits you have
successfully guessed, the number of digits which are in the combination
but misplaced, and the number of digits that are not in the combination
at all.

Combination are made of 4 digits from 0 to 5.
The same digit can not appear multiple times in the combination.
Enter 'q' or 'quit' to quit at any moment.

Enter a guess: 0123
1 found, 2 misplaced, 1 irrelevant; 9 guesses remaining.
Enter a guess: 2345
1 found, 2 misplaced, 1 irrelevant; 8 guesses remaining.
Enter a guess: nope
You must enter exactly 4 digits.
Enter a guess: quit
The combination was 2403.
$ 
```
