use std::str::FromStr;

const TURNS: &str = include_str!("./2.txt");

#[derive(PartialEq)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Pick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "x" | "a" => Ok(Self::Rock),
            "y" | "b" => Ok(Self::Paper),
            "z" | "c" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Pick {
    fn get_value(&self) -> u8 {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissors => 3,
        }
    }

    fn does_beat(&self, opponent: &Self) -> bool {
        match self {
            Self::Rock => opponent == &Self::Scissors,
            Self::Paper => opponent == &Self::Rock,
            Self::Scissors => opponent == &Self::Paper,
        }
    }
}


fn calculate_turn_points(opponent_pick: Pick, your_pick: Pick) -> u8 {
    let mut win_points: u8 = 0;
    if opponent_pick == your_pick {
        win_points = 3;
    } else if your_pick.does_beat(&opponent_pick) {
        win_points = 6;
    }

    win_points + your_pick.get_value()
}

fn main() {
    let total_points = TURNS
        .lines()
        .map(|line| -> u32 {
            let picks: Vec<&str> = line.split_whitespace().collect();

            calculate_turn_points(
                Pick::from_str(picks[0]).expect("non expected character"),
                Pick::from_str(picks[1]).expect("non expected character"),
            )
            .into()
        })
        .sum::<u32>();

    println!("first part: {:?}", total_points)
}

