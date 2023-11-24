use std::{fs, io::Result};

#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug)]
enum GameResult {
    Draw,
    Lose,
    Win,
}

impl GameResult {
    fn score(&self) -> usize {
        return match self {
            GameResult::Win => 6,
            GameResult::Lose => 0,
            GameResult::Draw => 3,
        };
    }
}

impl Move {
    fn score(&self) -> usize {
        return match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        };
    }
}

#[derive(Debug)]
struct Player1(Move);
#[derive(Debug)]
struct Player2(Move);

trait Player {
    fn new(play: &str) -> Move;
    fn default() -> Move;
}

impl Player for Player1 {
    fn new(play: &str) -> Move {
        return match play {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissor,
            _ => Move::Rock,
        };
    }

    fn default() -> Move {
        return Move::Rock;
    }
}

impl Player for Player2 {
    fn new(play: &str) -> Move {
        return match play {
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissor,
            _ => Move::Rock,
        };
    }

    fn default() -> Move {
        return Move::Rock;
    }
}

#[derive(Debug)]
struct Round(Move, Move);

impl Round {
    fn new(_roud: (&str, &str)) -> Self {
        Self(Player1::new(_roud.0), Player2::new(_roud.1))
    }

    fn default() -> Self {
        Self(Player1::default(), Player2::default())
    }

    fn score(&self) -> usize {
        let result = match self {
            Round(Move::Rock, Move::Rock) => GameResult::Draw,
            Round(Move::Rock, Move::Paper) => GameResult::Win,
            Round(Move::Rock, Move::Scissor) => GameResult::Lose,
            Round(Move::Paper, Move::Rock) => GameResult::Lose,
            Round(Move::Paper, Move::Paper) => GameResult::Draw,
            Round(Move::Paper, Move::Scissor) => GameResult::Win,
            Round(Move::Scissor, Move::Rock) => GameResult::Win,
            Round(Move::Scissor, Move::Paper) => GameResult::Lose,
            Round(Move::Scissor, Move::Scissor) => GameResult::Draw,
        };

        return result.score() + self.1.score();
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("./data/2.txt")?;

    let game: Vec<usize> = file
        .split("\n")
        .map(|line| {
            let parsed = line.split_once(" ").map(|item| Round::new(item));

            if let Some(result) = parsed {
                return result.score();
            }

            return 0;
        })
        .collect();

    println!("{:?}", game);

    let sum: usize = game.iter().sum();

    println!("{}", sum);

    Ok(())
}
