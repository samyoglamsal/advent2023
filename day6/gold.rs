use std::{fs, iter::zip};

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't open file.");

    let (_, time) = input_file.lines().nth(0).expect("Unable to parse first line.").split_once(":").unwrap();
    let time = time.replace(" ", "").parse::<u64>().unwrap();

    let (_, distance) = input_file.lines().nth(1).expect("Unable to parse first line.").split_once(":").unwrap();
    let distance = distance.replace(" ", "").parse::<u64>().unwrap();

    let mut ways = 0;
    for i in 0..=time {
        let distance_traveled = i * (time - i);
        if distance_traveled > distance {
            ways += 1;
        }
    }

    println!("{:?}", ways);
}
