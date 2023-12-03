use std::{fs, collections::HashMap};

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't open input file.");
    let mut grid = input_file.lines()
                         .map(|line| line.chars().collect::<Vec<char>>())
                         .collect::<Vec<Vec<char>>>();
    let mut gear_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();    
    let mut sum: u32 = 0;


    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j].is_digit(10) {
                let mut number: String = String::new();
                number.push(grid[i][j]);
                let mut has_touched_symbol = touches_symbol(&grid, i, j);

                // Check right 2x, replace with periods if digits
                if j < grid[0].len() - 1 {
                    if grid[i][j + 1].is_digit(10) {
                        number.push(grid[i][j + 1]);
                        has_touched_symbol = if has_touched_symbol.is_some() {has_touched_symbol} else {touches_symbol(&grid, i, j + 1)}; 
                        grid[i][j + 1] = '.';

                        if j < grid[0].len() - 2 {
                            if grid[i][j + 2].is_digit(10) {
                                number.push(grid[i][j + 2]);
                                has_touched_symbol = if has_touched_symbol.is_some() {has_touched_symbol} else {touches_symbol(&grid, i, j + 2)}; 
                                grid[i][j + 2] = '.';
                            }
                        }
                    }
                }

                let number = number.parse::<u32>().unwrap();
                match has_touched_symbol {
                    Some((i, j)) => {
                        let vector = gear_numbers.get_mut(&(i, j));
                        match vector {
                            Some(gear_parts) => {
                                gear_parts.push(number);
                            },
                            None => {
                                let mut gear_parts: Vec<u32> = vec!(number);
                                gear_numbers.insert((i, j), gear_parts);
                            }
                        }        
                    },
                    None => ()
                }
            }
        }
    }

    for vector in gear_numbers.values() {
        println!("{:?}", vector);
        if vector.len() != 2 {
            continue;
        }

        let mut product: u32 = 1;
        for element in vector {
            product *= element;
        }
        sum += product;
    }

    println!("{}", sum);
}

fn touches_symbol(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(usize, usize)> {
    // Check up
    if i > 0 && grid[i - 1][j] == '*' {
        return Some((i - 1, j));
    }
    // Check down
    if i < grid.len() - 1 && grid[i + 1][j] == '*' {
        return Some((i + 1, j));
    }
    // Check left
    if j > 0 && grid[i][j - 1] == '*' {
        return Some((i, j - 1));
    }
    // Check right
    if j < grid[0].len() - 1 && grid[i][j + 1] == '*' {
        return Some((i, j + 1));
    }
    // Check up left
    if i > 0 && j > 0 && grid[i - 1][j - 1] == '*' {
        return Some((i - 1, j - 1));
    }
    // Check up right
    if i > 0 && j < grid[0].len() - 1 && grid[i - 1][j + 1] == '*' {
        return Some((i - 1, j + 1));
    }
    // Check down left
    if i < grid.len() - 1 && j > 0 &&  grid[i + 1][j - 1] == '*' {
        return Some((i + 1, j - 1));
    }
    // Check down right
    if i < grid.len() - 1 && j < grid[0].len() - 1 && grid[i + 1][j + 1] == '*' {
        return Some((i + 1, j + 1));
    }

    return None;
}