pub struct Combination {
    values: Vec<u8>,
}

impl Combination {
    fn new(values: Vec<u8>) -> Combination {
        Combination { values: values }
    }
}

#[derive(Default)]
struct GuessResult {
    well_places: u8,
    misplaced: u8,
    not_in_combination: u8,
}

fn check(guess: &Combination, reference: &Combination) -> GuessResult {
    let mut result = GuessResult::default();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let reference = Combination::new(vec![1, 2, 3, 4]);
        let guess = Combination::new(vec![1, 2, 3, 4]);

        let result = check(&guess, &reference);

        assert_eq!(result.well_places, 0);
        assert_eq!(result.misplaced, 0);
        assert_eq!(result.not_in_combination, 0);
    }
}
