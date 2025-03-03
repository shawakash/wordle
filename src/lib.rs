pub mod algos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    // Green
    Correct,
    // Yellow
    Misplaced,
    // Gray
    Wrong,
}

pub struct Guess {
    word: String,
    mask: [Correctness; 5],
}

impl Correctness {
    fn compute(answer: &'static str, guess: &str) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut correctness = [Self::Wrong; 5];

        let mut answer_counts = [0; 26];
        for c in answer.chars() {
            answer_counts[c as usize - 'a' as usize] += 1;
        }

        for (i, (g, a)) in guess.chars().zip(answer.chars()).enumerate() {
            if g == a {
                correctness[i] = Self::Correct;
                answer_counts[g as usize - 'a' as usize] -= 1;
            }
        }

        for (i, g) in guess.chars().enumerate() {
            if correctness[i] == Self::Correct {
                continue;
            }
            let idx = g as usize - 'a' as usize;
            if answer_counts[idx] > 0 {
                correctness[i] = Self::Misplaced;
                answer_counts[idx] -= 1;
            }
        }

        correctness
    }
}

pub trait Guesser {
    // takes a list of previous guesses and returns a new guess
    fn guess(&mut self, prev_words: &[Guess]) -> String;
}

// logically it should have guessed only for 6 iterations, but we are benchmarking
pub fn play<G: Guesser>(answer: &'static str, guesser: &mut G) -> Option<usize> {
    let mut history = Vec::new();

    for i in 1..32 {
        let guess = guesser.guess(&history[..]);
        if guess == answer {
            return Some(i);
        }

        let correctness = Correctness::compute(answer, &guess);

        history.push(Guess {
            word: guess,
            mask: correctness,
        });
    }
    None
}

#[cfg(test)]
mod test {
    mod compute {
        use crate::Correctness;

        macro_rules! mask {
            (C) => {Correctness::Correct};
            (c) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (m) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            (w) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

        #[test]
        fn test_all_correct() {
            let result = Correctness::compute("hello", "hello");
            assert_eq!(result, mask![C C C C C]);
        }

        #[test]
        fn test_all_wrong() {
            let result = Correctness::compute("hello", "world");
            assert_eq!(result, mask![W M W C W]);
        }

        #[test]
        fn test_multiple_letters() {
            let result = Correctness::compute("hello", "lloll");
            assert_eq!(result, mask!(M W M C W));

            let result = Correctness::compute("world", "robot");
            assert_eq!(result, mask!(M C W W W));

            let result = Correctness::compute("peeks", "lever");
            assert_eq!(result, mask!(W C W M W));
        }

        #[test]
        fn test_duplicate_letters() {
            let result = Correctness::compute("hello", "llama");
            assert_eq!(result, mask![M M W W W]);

            let result = Correctness::compute("hello", "helps");
            assert_eq!(result, mask![c c c w w]);
        }

        #[test]
        fn test_misplaced_letters() {
            let result = Correctness::compute("hello", "ohell");
            assert_eq!(result, mask!(M M M C M));
        }

        #[test]
        fn test_mixed_correctness() {
            let result = Correctness::compute("speed", "spell");
            assert_eq!(result, mask!(C C C W W));
        }

        #[test]
        fn test_same_letter_different_positions() {
            let result = Correctness::compute("peaks", "spell");
            assert_eq!(result, mask!(M M M W W));
        }

        #[test]
        #[should_panic]
        fn test_invalid_length_answer() {
            Correctness::compute("toolong", "hello");
        }

        #[test]
        #[should_panic]
        fn test_invalid_length_guess() {
            Correctness::compute("hello", "toolong");
        }

        #[test]
        fn test_edge_cases() {
            // All same letters
            let result = Correctness::compute("aaaaa", "aaaaa");
            assert_eq!(result, mask![C C C C C]);

            // No matching letters
            let result = Correctness::compute("aaaaa", "bbbbb");
            assert_eq!(result, mask![W W W W W]);
        }
    }
}
