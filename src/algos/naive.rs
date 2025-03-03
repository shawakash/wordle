use crate::{Guess, Guesser};

#[derive(Clone)]
pub struct Naive;

impl Naive {
    pub fn new() -> Self {
        Naive
    }
}

impl Guesser for Naive {
    fn guess(&mut self, _prev_words: &[Guess]) -> String {
        todo!()
    }
}
