use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Combination {
    values: Vec<u8>,
}

impl Combination {
    fn len(&self) -> usize {
        self.values.len()
    }

    fn pieces(&self) -> std::iter::Enumerate<std::slice::Iter<u8>> {
        self.values.iter().enumerate()
    }

    fn piece(&self, pos: usize) -> u8 {
        self.values[pos]
    }

    fn position(&self, searched_piece: &u8, ignored_pos: &HashSet<usize>) -> Option<usize> {
        for (pos, piece) in self.pieces() {
            if piece == searched_piece && !ignored_pos.contains(&pos) {
                return Some(pos);
            }
        }

        None
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct GuessResult {
    ok: u8,
    misplaced: u8,
    ko: u8,
}

pub fn check(guess: &Combination, reference: &Combination) -> GuessResult {
    assert!(guess.len() == reference.len());

    let mut result = GuessResult::default();

    let mut used_guess_pieces = HashSet::new();
    let mut used_reference_pieces = HashSet::new();

    for (pos, piece) in guess.pieces() {
        if piece == &reference.piece(pos) {
            used_guess_pieces.insert(pos);
            used_reference_pieces.insert(pos);
            result.ok += 1;
        }
    }

    for (pos, piece) in guess.pieces() {
        if used_guess_pieces.contains(&pos) {
            continue;
        }

        match reference.position(piece, &used_reference_pieces) {
            None => continue,
            Some(ref_pos) => {
                used_guess_pieces.insert(pos);
                used_reference_pieces.insert(ref_pos);
                result.misplaced += 1;
            }
        }
    }

    for (pos, _piece) in guess.pieces() {
        if used_guess_pieces.contains(&pos) {
            continue;
        }

        used_guess_pieces.insert(pos);
        result.ko += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Combination {
        fn new(values: Vec<u8>) -> Combination {
            Combination { values: values }
        }
    }

    fn working_samples() -> Vec<(&'static str, Combination, Combination, GuessResult)> {
        let mut samples = Vec::new();

        samples.push((
            "Successful guess",
            Combination::new(vec![1, 2, 3, 4]),
            Combination::new(vec![1, 2, 3, 4]),
            GuessResult {
                ok: 4,
                misplaced: 0,
                ko: 0,
            },
        ));

        samples.push((
            "Fully unsuccessful guess",
            Combination::new(vec![5, 6, 7, 8]),
            Combination::new(vec![1, 2, 3, 4]),
            GuessResult {
                ok: 0,
                misplaced: 0,
                ko: 4,
            },
        ));

        samples.push((
            "Fully misplaced guess",
            Combination::new(vec![4, 3, 2, 1]),
            Combination::new(vec![1, 2, 3, 4]),
            GuessResult {
                ok: 0,
                misplaced: 4,
                ko: 0,
            },
        ));

        samples.push((
            "Same piece",
            Combination::new(vec![3, 3, 3, 3]),
            Combination::new(vec![1, 2, 3, 4]),
            GuessResult {
                ok: 1,
                misplaced: 0,
                ko: 3,
            },
        ));

        samples.push((
            "Mix 1",
            Combination::new(vec![1, 3, 4, 5]),
            Combination::new(vec![1, 2, 3, 4]),
            GuessResult {
                ok: 1,
                misplaced: 2,
                ko: 1,
            },
        ));

        samples.push((
            "Mix 2",
            Combination::new(vec![2, 2, 4, 2]),
            Combination::new(vec![1, 2, 3, 4]),
            GuessResult {
                ok: 1,
                misplaced: 1,
                ko: 2,
            },
        ));

        samples.push((
            "Same piece in reference combination #1",
            Combination::new(vec![1, 2, 3, 4]),
            Combination::new(vec![2, 2, 2, 2]),
            GuessResult {
                ok: 1,
                misplaced: 0,
                ko: 3,
            },
        ));

        samples.push((
            "Same piece in reference combination #2",
            Combination::new(vec![1, 2, 2, 4]),
            Combination::new(vec![2, 2, 2, 2]),
            GuessResult {
                ok: 2,
                misplaced: 0,
                ko: 2,
            },
        ));

        samples.push((
            "Same piece in reference combination #3",
            Combination::new(vec![3, 2, 3, 2]),
            Combination::new(vec![2, 2, 3, 3]),
            GuessResult {
                ok: 2,
                misplaced: 2,
                ko: 0,
            },
        ));

        samples
    }

    #[test]
    fn it_computes_the_appropriate_result() {
        for (desc, guess, reference, expected_result) in working_samples() {
            let actual_result = check(&guess, &reference);

            assert!(
                actual_result == expected_result,
                "Unexpected result for sample \"{}\": {:?}; expected {:?}",
                desc,
                actual_result,
                expected_result,
            );
        }
    }

    #[test]
    #[should_panic]
    fn it_panics_when_combinations_are_of_different_lengths() {
        let reference = Combination::new(vec![1]);
        let guess = Combination::new(vec![1, 2]);

        let _result = check(&guess, &reference);
    }
}
