use super::Combination;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::HashSet;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct GuessResult {
    pub ok: u8,
    pub misplaced: u8,
    pub ko: u8,
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

    #[test]
    fn check_computes_appropriate_results() {
        for (desc, guess, reference, expected_result) in check_samples() {
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
    fn check_panics_with_combinations_of_different_lengths() {
        let reference = Combination::from_slice(&[1]);
        let guess = Combination::from_slice(&[1, 2]);

        let _result = check(&guess, &reference);
    }

    fn check_samples() -> Vec<(&'static str, Combination, Combination, GuessResult)> {
        let mut samples = Vec::new();

        samples.push((
            "Successful guess",
            Combination::from_slice(&[1, 2, 3, 4]),
            Combination::from_slice(&[1, 2, 3, 4]),
            GuessResult {
                ok: 4,
                misplaced: 0,
                ko: 0,
            },
        ));

        samples.push((
            "Fully unsuccessful guess",
            Combination::from_slice(&[5, 6, 7, 8]),
            Combination::from_slice(&[1, 2, 3, 4]),
            GuessResult {
                ok: 0,
                misplaced: 0,
                ko: 4,
            },
        ));

        samples.push((
            "Fully misplaced guess",
            Combination::from_slice(&[4, 3, 2, 1]),
            Combination::from_slice(&[1, 2, 3, 4]),
            GuessResult {
                ok: 0,
                misplaced: 4,
                ko: 0,
            },
        ));

        samples.push((
            "Same piece",
            Combination::from_slice(&[3, 3, 3, 3]),
            Combination::from_slice(&[1, 2, 3, 4]),
            GuessResult {
                ok: 1,
                misplaced: 0,
                ko: 3,
            },
        ));

        samples.push((
            "Mix 1",
            Combination::from_slice(&[1, 3, 4, 5]),
            Combination::from_slice(&[1, 2, 3, 4]),
            GuessResult {
                ok: 1,
                misplaced: 2,
                ko: 1,
            },
        ));

        samples.push((
            "Mix 2",
            Combination::from_slice(&[2, 2, 4, 2]),
            Combination::from_slice(&[1, 2, 3, 4]),
            GuessResult {
                ok: 1,
                misplaced: 1,
                ko: 2,
            },
        ));

        samples.push((
            "Same piece in reference combination #1",
            Combination::from_slice(&[1, 2, 3, 4]),
            Combination::from_slice(&[2, 2, 2, 2]),
            GuessResult {
                ok: 1,
                misplaced: 0,
                ko: 3,
            },
        ));

        samples.push((
            "Same piece in reference combination #2",
            Combination::from_slice(&[1, 2, 2, 4]),
            Combination::from_slice(&[2, 2, 2, 2]),
            GuessResult {
                ok: 2,
                misplaced: 0,
                ko: 2,
            },
        ));

        samples.push((
            "Same piece in reference combination #3",
            Combination::from_slice(&[3, 2, 3, 2]),
            Combination::from_slice(&[2, 2, 3, 3]),
            GuessResult {
                ok: 2,
                misplaced: 2,
                ko: 0,
            },
        ));

        samples
    }
}
