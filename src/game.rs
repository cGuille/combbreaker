use super::check::check;
use super::check::GuessResult;
use super::ruleset::RuleSet;
use super::Combination;

#[derive(Default)]
pub struct Game {
    ruleset: RuleSet,
    combination: Option<Combination>,
    guess_count: u8,
    success: bool,
}

impl Game {
    pub fn new() -> Self {
        Game::default()
    }

    pub fn ruleset(&self) -> &RuleSet {
        &self.ruleset
    }

    pub fn remaining_guesses(&self) -> u8 {
        self.ruleset.max_tries - self.guess_count
    }

    pub fn combination(&self) -> &Option<Combination> {
        &self.combination
    }

    pub fn set_combination(&mut self, combination: Combination) -> Result<(), String> {
        if self.combination.is_some() {
            return Err("Combination already defined".to_string());
        }

        if combination.len() != self.ruleset.combination_size {
            return Err(format!(
                "Invalid combination: expected {} pieces",
                self.ruleset.combination_size
            ));
        }

        if self.ruleset.nb_pieces < combination.distinct_len() {
            return Err(format!(
                "Invalid combination: only {} pieces are allowed",
                self.ruleset.nb_pieces
            ));
        }

        if !self.ruleset.allow_repeat && combination.has_duplicate() {
            return Err("Combination is not allow to have duplicate".to_string());
        }

        self.combination = Some(combination);

        Ok(())
    }

    pub fn guess(&mut self, guess: &Combination) -> Result<GuessResult, String> {
        if self.ended() {
            return Err("Cannot guess because the game already ended".to_string());
        }

        self.guess_count += 1;

        match &self.combination {
            None => Err("Cannot guess without reference combination".to_string()),
            Some(reference) => {
                let guess_result = check(guess, &reference);

                if usize::from(guess_result.ok) == self.ruleset.combination_size {
                    self.success = true;
                }

                Ok(guess_result)
            }
        }
    }

    pub fn won(&self) -> bool {
        self.success
    }

    pub fn lost(&self) -> bool {
        self.guess_count >= self.ruleset.max_tries
    }

    pub fn ended(&self) -> bool {
        self.won() || self.lost()
    }

    pub fn score(&self) -> u8 {
        self.guess_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        assert_eq!(Game::new().ruleset, RuleSet::default());
    }

    #[test]
    fn test_game() -> Result<(), String> {
        let mut game = Game::new();

        assert!(!game.ended());

        let combination_to_guess = Combination::from_slice(&[1, 2, 3, 4]);
        game.set_combination(combination_to_guess.clone())?;

        assert!(!game.ended());

        let result = game.guess(&Combination::from_slice(&[1, 1, 1, 1]))?;
        assert_eq!(
            result,
            GuessResult {
                ok: 1,
                misplaced: 0,
                ko: 3
            }
        );

        assert!(!game.ended());

        let result = game.guess(&Combination::from_slice(&[1, 2, 3, 4]))?;
        assert_eq!(
            result,
            GuessResult {
                ok: 4,
                misplaced: 0,
                ko: 0
            }
        );

        assert!(game.ended());
        assert!(game.won());
        assert!(!game.lost());
        assert_eq!(game.combination().as_ref().unwrap(), &combination_to_guess);
        assert_eq!(game.score(), 2);

        Ok(())
    }
}
