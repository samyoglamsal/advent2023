use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Can't read input file.");
    let mut sum: u32 = 0;

    for line in input_file.lines() {
        let last_digit: u32 = line.chars().rfind(|x| x.is_digit(10)).unwrap().to_digit(10).unwrap();
        let first_digit: u32 = line.chars().find(|x| x.is_digit(10)).unwrap().to_digit(10).unwrap();


        sum += (first_digit * 10) + last_digit;
    }

    println!("{}", sum);
}