#[derive(Debug, PartialEq, Eq)]
pub struct RuleSet {
    // Define how many pieces there are in a combination
    pub combination_size: usize,

    // Define how many different pieces are available
    pub nb_pieces: usize,

    // Define how many guesses can be sent
    pub max_tries: u8,

    // Define whether the same piece can be used multiple times in the reference combination
    pub allow_repeat: bool,
}

impl Default for RuleSet {
    fn default() -> Self {
        RuleSet {
            combination_size: 4,
            nb_pieces: 6,
            max_tries: 10,
            allow_repeat: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruleset() {
        let ruleset = RuleSet::default();

        assert_eq!(ruleset.nb_pieces, 6);
        assert_eq!(ruleset.combination_size, 4);
        assert_eq!(ruleset.max_tries, 10);
        assert_eq!(ruleset.allow_repeat, false);
    }
}
