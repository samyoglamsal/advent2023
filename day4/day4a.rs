use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Could not open file.");
    let mut sum = 0;

    for line in input_file.lines() {
        let (_, numbers) = line.split_once(":").unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once("|").unwrap();

        let winning_numbers: Vec<&str> = winning_numbers.trim().split(" ").filter(|s| s != &"").collect::<Vec<&str>>();
        let my_numbers: Vec<&str> = my_numbers.trim().split(" ").filter(|s| s != &"").collect::<Vec<&str>>();

        let mut exponential_growth = 0;

        for num in my_numbers {
            if winning_numbers.contains(&num) {
                if exponential_growth == 0 {
                    exponential_growth = 1;
                } else {
                    exponential_growth *= 2;
                }
            }
        }

        sum += exponential_growth;
    }

    println!("{}", sum);
}
