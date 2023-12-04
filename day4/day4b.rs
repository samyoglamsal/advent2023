use std::{fs, collections::HashMap};

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let mut copies: HashMap<u32, u32> = HashMap::new();
    for val in 1..=198 {
        copies.insert(val, 1);
    }
    let mut sum = 0;
    let mut game_num = 1;

    for line in input_file.lines() {
        let (_, numbers) = line.split_once(":").unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once("|").unwrap();

        let winning_numbers: Vec<&str> = winning_numbers.trim().split(" ").filter(|s| s != &"").collect::<Vec<&str>>();
        let my_numbers: Vec<&str> = my_numbers.trim().split(" ").filter(|s| s != &"").collect::<Vec<&str>>();

        let mut num_matches = 0;
        for num in my_numbers {
            if winning_numbers.contains(&num) {
                num_matches += 1;
            }
        }

        let current_card_copies = *copies.get(&(game_num as u32)).unwrap();
        game_num += 1;

        for val in game_num..(game_num + num_matches) {
            copies.entry(val).and_modify(|v| *v += current_card_copies);
        }
    }

    println!("{}", copies.values().sum::<u32>());
}
