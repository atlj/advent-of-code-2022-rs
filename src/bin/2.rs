use std::str::FromStr;

const TURNS: &str = include_str!("./2.txt");

#[derive(Clone, PartialEq)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone)]
enum Outcome {
    Win,
    Lose,
    Tie,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.to_lowercase().as_str() {
            "x" => Ok(Self::Lose),
            "y" => Ok(Self::Tie),
            "z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl Outcome {
    fn guess_your_pick(&self, opponent_pick: &Pick) -> Pick {
        match self {
            Outcome::Win => opponent_pick.loses_to(),
            Outcome::Lose => opponent_pick.wins_to(),
            Outcome::Tie => opponent_pick.clone(),
        }
    }
}

impl FromStr for Pick {
    type Err = ();

    fn from_str(str: &str) -> Result<Pick, Self::Err> {
        match str.to_lowercase().as_str() {
            "a" | "x" => Ok(Pick::Rock),
            "b" | "y" => Ok(Pick::Paper),
            "c" | "z" => Ok(Pick::Scissors),
            _ => Err(()),
        }
    }
}

impl Pick {
    fn wins_to(&self) -> Pick {
        match self {
            Pick::Rock => Self::Scissors,
            Pick::Paper => Self::Rock,
            Pick::Scissors => Self::Paper,
        }
    }

    fn loses_to(&self) -> Pick {
        match self {
            Pick::Rock => Self::Paper,
            Pick::Paper => Self::Scissors,
            Pick::Scissors => Self::Rock,
        }
    }

    fn get_value(&self) -> u32 {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissors => 3,
        }
    }

    fn does_beat(&self, opponent_pick: Pick) -> bool {
        opponent_pick == self.wins_to()
    }

    fn does_lose(&self, opponent_pick: Pick) -> bool {
        opponent_pick == self.loses_to()
    }
}

fn calculate_turn_points(your_pick: Pick, opponent_pick: Pick) -> u32 {
    let mut win_points: u32 = 0;
    if opponent_pick == your_pick {
        win_points = 3;
    } else if your_pick.does_beat(opponent_pick) {
        win_points = 6;
    }

    win_points + your_pick.get_value()
}

fn main() {
    let total_points_1 = TURNS
        .lines()
        .map(|line| -> u32 {
            let picks: Vec<&str> = line.split_whitespace().collect();

            calculate_turn_points(
                Pick::from_str(picks[1]).expect("non expected character"),
                Pick::from_str(picks[0]).expect("non expected character"),
            )
            .into()
        })
        .sum::<u32>();

    let total_points_2 = TURNS
        .lines()
        .map(|line| -> u32 {
            let picks: Vec<&str> = line.split_whitespace().collect();

            let opponent_pick = Pick::from_str(picks[0]).expect("non expected character");
            let outcome = Outcome::from_str(picks[1]).expect("non expected character");
            let your_pick = outcome.guess_your_pick(&opponent_pick);

            calculate_turn_points(your_pick, opponent_pick).into()
        })
        .sum::<u32>();

    println!("first part: {:?}", total_points_1);
    println!("second part: {:?}", total_points_2)
}
