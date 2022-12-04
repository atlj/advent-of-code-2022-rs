const COMPARTMENTS: &str = include_str!("./3.txt");

fn calculate_item_priority(item: char) -> u8 {
    let mut byte_code = [0; 1];
    item.encode_utf8(&mut byte_code);
    if item.is_uppercase() {
        return byte_code[0] - 38;
    } else {
        return byte_code[0] - 96;
    }
}

fn find_repeating(compartment: &str) -> Result<char, ()> {
    let halves = compartment.split_at(compartment.len() / 2);
    let first_half = halves.0;
    let second_half = halves.1;

    for item in first_half.chars() {
        if second_half.contains(item) {
            return Ok(item);
        }
    }
    Err(())
}

fn main() {
    let priorities_1: u16 = COMPARTMENTS
        .lines()
        .map(|compartment| -> u16 {
            calculate_item_priority(find_repeating(compartment).unwrap()).into()
        })
        .sum::<u16>();

    let mut count = 0;
    let mut group = [""; 3];
    let mut solution2: u32 = 0;
    for compartment in COMPARTMENTS.lines() {
        if count < 3 {
            group[count] = compartment;
            count += 1;
        }
        if count == 3 {
            for item in group[0].chars() {
                if group[1].contains(item) && group[2].contains(item) {
                    solution2 += calculate_item_priority(item) as u32;
                    break;
                }
            }

            count = 0
        }
    }

    println!("Solution for the first half: {:?}", priorities_1);
    println!("Solution for the second half: {:?}", solution2)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn item_priority() {
        assert_eq!(calculate_item_priority('a'), 1);
        assert_eq!(calculate_item_priority('A'), 27)
    }

    #[test]
    fn repeates() {
        assert_eq!(find_repeating("aBca").unwrap(), 'a')
    }
}
