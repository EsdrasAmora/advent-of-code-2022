use anyhow::bail;

#[derive(Debug, PartialEq)]
// #[repr(u8)]
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
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => bail!("Invalid Shape"),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt")
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|x| x.try_into().unwrap())
                .collect::<Vec<Shape>>()
        })
        .map(|x| -> [Shape; 2] { x.try_into().unwrap() })
        .map(|x| {
            let [a, b] = x;
            b.play(a)
        })
        .sum::<u32>();
    println!("{:?}", input);
}
