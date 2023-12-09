use std::collections::HashSet;
use std::fs::read_to_string;


struct Array2D<T> {
    arr: Vec<Vec<T>>,
    cols: usize,
    rows: usize,
}

impl<T: Copy> Array2D<T> {
    fn new(r: usize, c: usize, v: T) -> Self {
        Array2D { arr: vec![vec![v; c]; r], cols: c, rows: r }
    }

    fn get(&self, r: i16, c: i16) -> Option<&T> {
        if r < 0 || c < 0 {
            return None;
        }
        if let Some(r) = self.arr.get(r as usize) {
            if let Some(v) = r.get(c as usize) {
                return Some(v);
            }
        }
        None
    }

    fn set(&mut self, r: usize, c: usize, v: T) -> Result<(), ()> {
        if r >= self.rows || c >= self.cols {
            Err(())
        } else {
            self.arr[r][c] = v;
            Ok(())
        }
    }
}


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
    const AROUND: [(i16, i16); 8] = [
        (-1, 0), (1, 0), (0, -1), (0, 1), (1, 1), (-1, 1), (1, -1), (-1, -1)
    ];

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

    let mut table_2d = Array2D::new(nb_rows, nb_cols, 0);
    for r in 0..nb_rows {
        for c in 0..nb_cols {
            table_2d.set(r, c, table[r][c]).unwrap();
        }
    }

    for r in 0..nb_rows {
        for c in 0..nb_cols {
            if lines[r][c] == '*' {
                let mut numbers = HashSet::new();

                for (h, v) in AROUND {
                    if let Some(number) = table_2d.get(r as i16 + h, c as i16 + v) {
                        if *number != 0 {
                            numbers.insert(*number);
                        }
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
