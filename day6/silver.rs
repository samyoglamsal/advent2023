use std::{fs, iter::zip};

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't open file.");

    let (_, times) = input_file.lines().nth(0).expect("Unable to parse first line.").split_once(":").unwrap();
    let times = times.trim().split(" ").filter(|x| x != &"").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let (_, distances) = input_file.lines().nth(1).expect("Unable to parse second line.").split_once(":").unwrap();
    let distances = distances.trim().split(" ").filter(|x| x != &"").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut ways: Vec<u64> = vec!();

    for (time, distance) in zip(times, distances) {
        let mut count = 0; 

        for i in 0..=time {
            let distance_traveled = i * (time - i);
            if distance_traveled > distance {
                count += 1;
            }
        }

        ways.push(count);
    }

    let mut product = 1;
    for way in ways {
        product *= way;
    }

    println!("{}", product);
}
