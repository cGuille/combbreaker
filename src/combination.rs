use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq)]
pub struct Combination {
    values: Vec<u8>,
}

impl Combination {
    pub fn from_slice(pieces: &[u8]) -> Combination {
        pieces.to_vec().into_iter().collect()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn distinct_len(&self) -> usize {
        let mut pieces_set = HashSet::new();
        let mut count = 0;

        for piece in &self.values {
            if !pieces_set.contains(&piece) {
                count += 1;
                pieces_set.insert(piece);
            }
        }

        count
    }

    pub fn pieces(&self) -> std::iter::Enumerate<std::slice::Iter<u8>> {
        self.values.iter().enumerate()
    }

    pub fn piece(&self, pos: usize) -> u8 {
        self.values[pos]
    }

    pub fn position(&self, searched_piece: &u8, ignored_pos: &HashSet<usize>) -> Option<usize> {
        for (pos, piece) in self.pieces() {
            if piece == searched_piece && !ignored_pos.contains(&pos) {
                return Some(pos);
            }
        }

        None
    }

    pub fn has_duplicate(&self) -> bool {
        self.len() > self.distinct_len()
    }
}

impl FromIterator<u8> for Combination {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Combination {
            values: Vec::from_iter(iter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_iter() {
        let pieces = vec![1, 2, 3];
        let combination: Combination = pieces.clone().into_iter().collect();

        assert_eq!(combination.values, pieces);
    }

    #[test]
    fn test_from_slice() {
        let pieces = [1, 2, 3];
        let combination = Combination::from_slice(&pieces);

        assert_eq!(combination.values, pieces);
    }

    #[test]
    fn test_len() {
        let combination = Combination::from_slice(&[1, 2, 3]);
        assert_eq!(combination.len(), 3);

        let combination = Combination::from_slice(&[1, 2, 3, 4]);
        assert_eq!(combination.len(), 4);

        let combination = Combination::from_slice(&[1, 1, 1]);
        assert_eq!(combination.len(), 3);
    }

    #[test]
    fn test_distinct_len() {
        let combination = Combination::from_slice(&[1, 2, 3]);
        assert_eq!(combination.distinct_len(), 3);

        let combination = Combination::from_slice(&[1, 2, 3, 4]);
        assert_eq!(combination.distinct_len(), 4);

        let combination = Combination::from_slice(&[1, 1, 1]);
        assert_eq!(combination.distinct_len(), 1);
    }

    #[test]
    fn test_pieces() {
        let pieces = [1, 2, 3];
        let combination = Combination::from_slice(&pieces);

        let expected: Vec<(usize, &u8)> = pieces.iter().enumerate().collect();
        let actual: Vec<(usize, &u8)> = combination.pieces().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_piece() {
        let combination = Combination::from_slice(&[1, 2, 3]);

        assert_eq!(combination.piece(0), 1);
        assert_eq!(combination.piece(1), 2);
        assert_eq!(combination.piece(2), 3);
    }

    #[test]
    #[should_panic]
    fn test_piece_out_of_bounds() {
        Combination::from_slice(&[1, 2, 3]).piece(3);
    }

    #[test]
    fn test_position() {
        fn ignored(list: &[usize]) -> HashSet<usize> {
            list.iter().cloned().collect()
        }

        let combination = Combination::from_slice(&[1, 2, 3, 2]);

        let position = combination.position(&0, &ignored(&[]));
        assert!(position.is_none());

        let position = combination.position(&1, &ignored(&[]));
        assert_eq!(position, Some(0));

        let position = combination.position(&3, &ignored(&[]));
        assert_eq!(position, Some(2));

        let position = combination.position(&2, &ignored(&[0]));
        assert_eq!(position, Some(1));

        let position = combination.position(&2, &ignored(&[1]));
        assert_eq!(position, Some(3));
    }

    #[test]
    fn test_has_duplicate() {
        assert!(Combination::from_slice(&[1, 2, 3, 2]).has_duplicate());
        assert!(!Combination::from_slice(&[1, 2, 3]).has_duplicate());
    }
}
