use std::fs;

fn main() {
    let input_file = fs::read_to_string("input.txt").expect("Couldn't open input file.");
    let mut sum = 0;
    let mut grid = input_file.lines()
                         .map(|line| line.chars().collect::<Vec<char>>())
                         .collect::<Vec<Vec<char>>>();

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
                        has_touched_symbol = has_touched_symbol || touches_symbol(&grid, i, j + 1);
                        grid[i][j + 1] = '.';
                        if j < grid[0].len() - 2 {
                            if grid[i][j + 2].is_digit(10) {
                                number.push(grid[i][j + 2]);
                                has_touched_symbol = has_touched_symbol || touches_symbol(&grid, i, j + 2);
                                grid[i][j + 2] = '.';
                            }
                        }
                    }
                }

                if has_touched_symbol {
                    sum += number.parse::<u32>().unwrap();
                }
            }
        }
    }

    println!("{}", sum);
}

fn touches_symbol(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // Check up
    if i > 0 && grid[i - 1][j] != '.' && !grid[i - 1][j].is_digit(10) {
        return true;
    }
    // Check down
    if i < grid.len() - 1 && grid[i + 1][j] != '.' && !grid[i + 1][j].is_digit(10) {
        return true;
    }
    // Check left
    if j > 0 && grid[i][j - 1] != '.' && !grid[i][j - 1].is_digit(10) {
        return true;
    }
    // Check right
    if j < grid[0].len() - 1 && grid[i][j + 1] != '.' && !grid[i][j + 1].is_digit(10) {
        return true;
    }
    // Check up left
    if i > 0 && j > 0 && grid[i - 1][j - 1] != '.' && !grid[i - 1][j - 1].is_digit(10) {
        return true;
    }
    // Check up right
    if i > 0 && j < grid[0].len() - 1 && grid[i - 1][j + 1] != '.' && !grid[i - 1][j + 1].is_digit(10) {
        return true;
    }
    // Check down left
    if i < grid.len() - 1 && j > 0 &&  grid[i + 1][j - 1] != '.' && !grid[i + 1][j - 1].is_digit(10) {
        return true;
    }
    // Check down right
    if i < grid.len() - 1 && j < grid[0].len() - 1 && grid[i + 1][j + 1] != '.' && !grid[i + 1][j + 1].is_digit(10) {
        return true;
    }
    return false;
}