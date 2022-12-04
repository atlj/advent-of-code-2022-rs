use day2::Pick;
use std::str::FromStr;

const TURNS: &str = include_str!("./2.txt");

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
