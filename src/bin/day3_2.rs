use std::collections::HashSet;
use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<Vec<char>> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|s| s.chars().collect())  // make each slice into vec of chars
        .collect()  // gather them together into a vector
}


fn main() {
    let lines = read_lines("data/day3.txt");
    let nb_rows = lines.len();
    let nb_cols = lines[0].len();

    let mut table = vec![vec![0; nb_cols]; nb_rows];

    let mut ch: char;
    let mut number;
    let mut sum = 0;

    for r in 0..nb_rows {
        number = 0;
        let mut digit_start: usize = 0;
        let mut digit_end: usize = 0;
        for c in 0..nb_cols {
            ch = lines[r][c];
            if ch.is_digit(10) {
                if number == 0 {
                    digit_start = c;
                    digit_end = digit_start + 1;
                } else {
                    digit_end = c;
                }
                number = number * 10 + ch.to_digit(10).unwrap();
            }
            if !ch.is_digit(10) || c == nb_cols - 1 {
                if number != 0 {
                    for dc in digit_start..=digit_end {
                        table[r][dc] = number;
                    }
                    number = 0;
                }
            }
        }
    }

    for r in 0..nb_rows {
        for c in 0..nb_cols {
            if lines[r][c] == '*' {
                let mut numbers = HashSet::new();
                if r != 0 {
                    number = table[r-1][c];
                    if number != 0 {
                        numbers.insert(number);
                    }
                }
                if r != nb_rows - 1 {
                    number = table[r+1][c];
                    if number != 0 {
                        numbers.insert(number);
                    }
                }
                if c != 0 {
                    number = table[r][c-1];
                    if number != 0 {
                        numbers.insert(number);
                    }
                }
                if c != nb_rows - 1 {
                    number = table[r][c+1];
                    if number != 0 {
                        numbers.insert(number);
                    }
                }
                if r != 0 && c != 0 {
                    number = table[r-1][c-1];
                    if number != 0 {
                        numbers.insert(number);
                    }
                }
                if r != 0 && c != nb_cols - 1 {
                    number = table[r-1][c+1];
                    if number != 0 {
                        numbers.insert(number);
                    }
                }
                if r != nb_rows - 1 && c != nb_cols - 1 {
                    number = table[r+1][c+1];
                    if number != 0 {
                        numbers.insert(number);
                    }
                }
                if r != nb_rows - 1 && c != 0 {
                    number = table[r+1][c-1];
                    if number != 0 {
                        numbers.insert(number);
                    }
                }

                if numbers.len() == 2 {
                    sum += numbers.iter().fold(1, |acc, x| acc * x);
                }
            }
        }
    }
    println!("sum of gears: {}", sum);
}
