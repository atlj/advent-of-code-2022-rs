use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
pub enum Pick {
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
    pub fn get_value(&self) -> u8 {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissors => 3,
        }
    }

    pub fn does_beat(&self, opponent: &Self) -> bool {
        opponent == self.winning_pick()
    }

    fn winning_pick(&self) -> &Self {
        match self {
            Self::Rock => &Self::Scissors,
            Self::Paper => &Self::Rock,
            Self::Scissors => &Self::Paper,
        }
    }

    fn losing_pick(&self) -> &Self {
        match self {
            Self::Rock => &Self::Paper,
            Self::Paper => &Self::Scissors,
            Self::Scissors => &Self::Rock,
        }
    }
}

#[derive(PartialEq)]
pub enum Outcome {
    Win,
    Draw,
    Lose,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "z" => Ok(Outcome::Win),
            "y" => Ok(Outcome::Draw),
            "x" => Ok(Outcome::Lose),
            _ => Err(()),
        }
    }
}

impl Outcome {
    pub fn guess_your_pick(&self, opponent: Pick) -> Pick {
        match self {
            Self::Draw => opponent,
            Self::Win => *opponent.losing_pick(),
            Self::Lose => *opponent.winning_pick(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_does_beat() {
        let your_pick = Pick::Rock;
        let opponent_pick = Pick::Scissors;

        assert!(your_pick.does_beat(&opponent_pick))
    }
}
