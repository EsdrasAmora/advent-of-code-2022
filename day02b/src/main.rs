use anyhow::bail;

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn desired_shape_against(&self, opponent: Shape) -> Shape {
        match opponent {
            Shape::Rock => match self {
                GameResult::Win => Shape::Paper,
                GameResult::Lose => Shape::Scissors,
                GameResult::Draw => Shape::Rock,
            },
            Shape::Paper => match self {
                GameResult::Win => Shape::Scissors,
                GameResult::Lose => Shape::Rock,
                GameResult::Draw => Shape::Paper,
            },
            Shape::Scissors => match self {
                GameResult::Win => Shape::Rock,
                GameResult::Lose => Shape::Paper,
                GameResult::Draw => Shape::Scissors,
            },
        }
    }
}

impl TryFrom<&str> for GameResult {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => bail!("Invalid GameResult, current: {}", value),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn play(self, opponent: Shape) -> u32 {
        let result = match self {
            Shape::Rock => match opponent {
                Shape::Rock => 3,
                Shape::Paper => 0,
                Shape::Scissors => 6,
            },
            Shape::Paper => match opponent {
                Shape::Rock => 6,
                Shape::Paper => 3,
                Shape::Scissors => 0,
            },
            Shape::Scissors => match opponent {
                Shape::Rock => 0,
                Shape::Paper => 6,
                Shape::Scissors => 3,
            },
        };
        self as u32 + result as u32
    }
}

impl TryFrom<&str> for Shape {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => bail!("Invalid Shape"),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|x| x.split(" ").collect::<Vec<&str>>())
        .map(|x| {
            let (opponent, game_result) = (
                Shape::try_from(x[0]).unwrap(),
                GameResult::try_from(x[1]).unwrap(),
            );
            game_result.desired_shape_against(opponent).play(opponent)
        })
        .sum::<u32>();
    println!("{:?}", input);
}
