fn main() {
    let elves_table = include_str!("./1.txt");

    let mut elf_callories: Vec<u32> = elves_table
        .split("\n\n")
        .map(|elf| -> u32 {
            elf.lines()
                .map(|callory| callory.parse::<u32>().expect("non numeric value is passed"))
                .sum()
        })
        .collect();

    elf_callories.sort_by(|a, b| b.cmp(a));
    let max = elf_callories[0];
    let sum_of_top_three: u32 = elf_callories.drain(0..3).sum();

    println!("first part: {max}");
    println!("second part: {:?}", sum_of_top_three)
}
