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

    fn score_part2(&self) -> usize {
        let game_result = match self.1 {
            Move::Rock => GameResult::Lose,
            Move::Paper => GameResult::Draw,
            Move::Scissor => GameResult::Win,
        };

        let result = match (&self.0, &game_result) {
            (Move::Rock, GameResult::Win) => Move::Paper,
            (Move::Rock, GameResult::Draw) => Move::Rock,
            (Move::Rock, GameResult::Lose) => Move::Scissor,
            (Move::Paper, GameResult::Lose) => Move::Rock,
            (Move::Paper, GameResult::Draw) => Move::Paper,
            (Move::Paper, GameResult::Win) => Move::Scissor,
            (Move::Scissor, GameResult::Win) => Move::Rock,
            (Move::Scissor, GameResult::Lose) => Move::Paper,
            (Move::Scissor, GameResult::Draw) => Move::Scissor,
        };

        return result.score() + game_result.score();
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("./data/2.txt")?;
    //let input = fs::read_to_string("./data/2.example.txt")?;

    let game: Vec<Round> = input
        .trim_end()
        .split("\n")
        .map(|line| {
            let parsed = line.split_once(" ").map(|item| Round::new(item));

            if let Some(result) = parsed {
                return result;
            }

            return Round::default();
        })
        .collect();

    let sum_part1: usize = game.iter().map(|x| x.score()).sum();
    let sum_part2: usize = game.iter().map(|x| x.score_part2()).sum();
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);

    Ok(())
}
