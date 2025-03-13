use wordle::{algos::naive::Naive, play};

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let mut game = Naive::new();

    for answer in GAMES.split_whitespace() {
        play(answer, &mut game);
    }
}
