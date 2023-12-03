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

    let mut ch: char;
    let mut sum = 0;

    for r in 0..nb_rows {
        let mut number = 0;
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
                        if r != 0 {
                            ch = lines[r-1][dc];
                            if ch != '.' && !ch.is_digit(10) {
                                sum += number;
                                break;
                            }
                        }
                        if r != nb_rows - 1 {
                            ch = lines[r+1][dc];
                            if ch != '.' && !ch.is_digit(10) {
                                sum += number;
                                break;
                            }
                        }
                        if dc != 0 {
                            ch = lines[r][dc-1];
                            if ch != '.' && !ch.is_digit(10) {
                                sum += number;
                                break;
                            }
                        }
                        if dc != nb_rows - 1 {
                            ch = lines[r][dc+1];
                            if ch != '.' && !ch.is_digit(10) {
                                sum += number;
                                break;
                            }
                        }
                        if r != 0 && dc != 0 {
                            ch = lines[r-1][dc-1];
                            if ch != '.' && !ch.is_digit(10) {
                                sum += number;
                                break;
                            }
                        }
                        if r != 0 && dc != nb_cols - 1 {
                            ch = lines[r-1][dc+1];
                            if ch != '.' && !ch.is_digit(10) {
                                sum += number;
                                break;
                            }
                        }
                        if r != nb_rows - 1 && dc != nb_cols - 1 {
                            ch = lines[r+1][dc+1];
                            if ch != '.' && !ch.is_digit(10) {
                                sum += number;
                                break;
                            }
                        }
                        if r != nb_rows - 1 && dc != 0 {
                            ch = lines[r+1][dc-1];
                            if ch != '.' && !ch.is_digit(10) {
                                sum += number;
                                break;
                            }
                        }
                    }
                    number = 0;
                }
            }
        }
    }

    println!("sum of part numbers: {}", sum);
}
