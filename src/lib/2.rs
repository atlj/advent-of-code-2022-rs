use std::str::FromStr;

#[derive(PartialEq)]
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
        match self {
            Self::Rock => opponent == &Self::Scissors,
            Self::Paper => opponent == &Self::Rock,
            Self::Scissors => opponent == &Self::Paper,
        }
    }
}
