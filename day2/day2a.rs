use std::{fs, collections::HashMap};

fn main() {
    let reds = 12;
    let greens = 13; 
    let blues = 14;
    let mut sum = 0;

    let input_file = fs::read_to_string("input.txt").expect("Could not open file.");

    // Parse each game
    'outer: for line in input_file.lines() {
        let mut iter = line.split(":");
        let game_num: u32 = iter.next().unwrap()[5..].parse().unwrap();

        let sets = iter.next().unwrap().split(";");
        for set in sets {
            let commaless_set = set.trim().replace(",", "");
            let mut tokens_iter = commaless_set.split(" ").peekable();

            while tokens_iter.peek().is_some() {
                let value = tokens_iter.next().unwrap().parse::<u32>().unwrap();
                let color = tokens_iter.next().unwrap(); 

                match color {
                    "green" => if value > greens {continue 'outer;},
                    "red" => if value > reds {continue 'outer;},
                    "blue" => if value > blues {continue 'outer;},
                    _ => println!("You're not supposed to be here.")
                }
            }
        }

        sum += game_num;
    }

    println!("{}", sum);
}