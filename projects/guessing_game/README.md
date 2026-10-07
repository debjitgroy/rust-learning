# guessing_game

Guess a random number between 1 and 100. You get 10 tries.

After each guess the program says whether it was too small or too big. It stops
when you guess right or run out of tries, then prints the secret number.

Input that isn't a number is skipped, but it still costs a try.

## Run

```bash
cargo run
```

## Dependencies

`rand` 0.10.1, for generating the secret number.
