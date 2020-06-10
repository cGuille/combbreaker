mod check;
mod combination;
mod game;
mod ruleset;

pub use combination::Combination;
pub use game::Game;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lib() -> Result<(), String> {
        let mut game = Game::new();

        game.define_combination(Combination::from_slice(&[1, 2, 3, 4]))?;

        let _ = game.guess(&Combination::from_slice(&[1, 2, 3, 4]));
        assert!(game.ended());

        Ok(())
    }
}
