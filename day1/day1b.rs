use std::{fs, collections::HashMap};

fn main() {
    let word_to_num = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);
    let input_file: String = fs::read_to_string("input.txt").expect("Can't read input file.");
    let mut sum = 0;

    'sus: for line in input_file.lines() {
        let (last_digit_index, last_digit_c): (usize, char) = line.char_indices().rfind(|(_, x)| x.is_digit(10)).unwrap();
        let (first_digit_index, first_digit_c): (usize, char) = line.char_indices().find(|(_, x)| x.is_digit(10)).unwrap();

        let first_digit = first_digit_c.to_digit(10).unwrap();
        let last_digit = last_digit_c.to_digit(10).unwrap();

        let mut first_word_digit = 0;
        let mut first_word_index = 1000;
        let mut last_word_digit = 0;
        let mut last_word_index = 0;


        'lol: for i in 0..line.len() {
            for key in word_to_num.keys() {
                if line[i..].starts_with(key) {
                    first_word_digit = word_to_num[key];
                    first_word_index = i;
                    break 'lol;
                }
            }
        }

        'lmao: for i in (0..line.len()).rev() {
            for key in word_to_num.keys() {
                if line[i..].starts_with(key) {
                    last_word_digit = word_to_num[key];
                    last_word_index = i;
                    break 'lmao;
                }
            }
        }
        println!("fw:{} lw:{} fd:{} ld:{}", first_word_digit, last_word_digit, first_digit, last_digit);

        if first_digit_index < first_word_index {
            if last_digit_index < last_word_index {
                sum += first_digit * 10 + last_word_digit;
                // println!("{} {}", first_digit, last_word_digit);
            } else {
                sum += first_digit * 10 + last_digit;
                // println!("{} {}", first_digit, last_digit);
            }
        } else {
            if last_digit_index < last_word_index {
                sum += first_word_digit * 10 + last_word_digit;
                // println!("{} {}", first_word_digit, last_word_digit);
            } else {
                sum += first_word_digit * 10 + last_digit;
                // println!("{} {}", first_word_digit, last_digit);
            }
        }
    }
    println!("{}", sum);
}