use std::fs;

fn main() {
    let reds = 12;
    let greens = 13; 
    let blues = 14;
    let mut sum = 0;
    let input_text = fs::read_to_string("input.txt").expect("Unable to open file.");

    for line in input_text.lines() {
        let sets = line.split(":").nth(1).unwrap().split(";");
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in sets {
            let tokens = set.trim().replace(",", "");
            let mut tokens_iter = tokens.split(" ").peekable();

            while tokens_iter.peek().is_some() {
                let value = tokens_iter.next().unwrap().parse::<u32>().unwrap();
                let color = tokens_iter.next().unwrap();

                println!("{}", color);

                match color {
                    "red" => if value > max_red {max_red = value;},                 
                    "green" => if value > max_green {max_green = value;},
                    "blue" => if value > max_blue {max_blue = value;},
                    _ => println!("You shouldn't be here.")
                }
            }
        }
        
        let power = max_red * max_blue * max_green;
        sum += power;
    }

    println!("{}", sum);
}