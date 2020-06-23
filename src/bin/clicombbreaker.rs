use combbreaker::Combination;
use combbreaker::Game;
use combbreaker::RuleSet;
use rand::distributions::{Distribution, Uniform};
use std::convert::TryInto;
use std::io;
use std::io::Write;
use std::str::FromStr;

const WELCOME_MESSAGE: &str = "
Welcome to COMBBREAKER, a game where you have to break a combination by
taking guesses. You will be answered with the number of digits you have
successfully guessed, the number of digits which are in the combination
but misplaced, and the number of digits that are not in the combination
at all.
";

fn main() -> Result<(), String> {
    println!("{}", WELCOME_MESSAGE);

    let mut game = Game::new();

    println!(
        "Combination are made of {} digits from 0 to {}.",
        game.ruleset().combination_size,
        game.ruleset().nb_pieces - 1,
    );
    println!(
        "The same digit {} appear multiple times in the combination.",
        if game.ruleset().allow_repeat { "can" } else { "can not" }
    );
    println!("Enter 'q' or 'quit' to quit at any moment.");
    println!("");

    game.set_combination(gen_combination(game.ruleset()))?;

    while !game.ended() {
        print!("Enter a guess: ");
        io::stdout().flush().unwrap();

        match read_guess(game.ruleset().combination_size) {
            GuessInput::Quit => break,
            GuessInput::AnythingElse => {
                println!(
                    "You must enter exactly {} digits.",
                    game.ruleset().combination_size
                );
                continue;
            }
            GuessInput::Guess(guess) => {
                let result = game.guess(&guess)?;
                println!(
                    "{} found, {} misplaced, {} irrelevant; {} guesses remaining.",
                    result.ok,
                    result.misplaced,
                    result.ko,
                    game.remaining_guesses()
                );
            }
        };
    }

    if game.won() {
        println!(
            "Congrats! You won with {} guess{}.",
            game.score(),
            if game.score() > 1 { "es" } else { "" }
        );
    } else if game.lost() {
        println!(
            "Sadly, you ran out of time… better luck next game! The combination was {}.",
            fmt_combination(game.combination())
        );
    } else {
        println!(
            "The combination was {}.",
            fmt_combination(game.combination())
        );
    }

    Ok(())
}

enum GuessInput {
    Guess(Combination),
    Quit,
    AnythingElse,
}

fn read_guess(size: usize) -> GuessInput {
    let mut input = String::new();

    let result = io::stdin().read_line(&mut input);

    if result.is_err() {
        return GuessInput::AnythingElse;
    }

    let input = input.trim();

    if input == "q" || input == "quit" {
        return GuessInput::Quit;
    }

    if input.len() != size {
        return GuessInput::AnythingElse;
    }

    let mut pieces = Vec::new();

    for c in input.chars() {
        match u8::from_str(&c.to_string()) {
            Ok(digit) => pieces.push(digit),
            Err(_) => return GuessInput::AnythingElse,
        }
    }

    GuessInput::Guess(pieces.into_iter().collect())
}

fn gen_combination(ruleset: &RuleSet) -> Combination {
    let mut pieces = Vec::new();
    let distribution = Uniform::from(0..ruleset.nb_pieces);
    let mut rng = rand::thread_rng();

    while pieces.len() < ruleset.combination_size {
        let piece = distribution.sample(&mut rng).try_into().unwrap();

        if !pieces.contains(&piece) {
            pieces.push(piece);
        }
    }

    Combination::from_slice(&pieces[..])
}

fn fmt_combination(combination: &Option<Combination>) -> String {
    match combination {
        None => "undefined".to_string(),
        Some(combination) => {
            let pieces = combination.pieces();
            format!("{}{}{}{}", pieces[0], pieces[1], pieces[2], pieces[3])
        }
    }
}
