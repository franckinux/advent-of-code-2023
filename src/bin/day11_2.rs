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
    let lines = read_lines("data/day11.txt");
    let nb_rows = lines.len();
    let nb_cols = lines[0].len();

    let mut lines_2d = Array2D::new(nb_rows, nb_cols, ' ');
    for r in 0..nb_rows {
        for c in 0..nb_cols {
            lines_2d.set(r, c, lines[r][c]).unwrap();
        }
    }

    // find empty rows and columns
    let mut empty_rows = Vec::new();
    for i in 0..nb_rows {
        if lines_2d.arr[i].as_slice().iter().all(|x| *x == '.') {
            empty_rows.push(i as i16);
        }
    }
    let mut empty_cols = Vec::new();
    for j in 0..nb_cols {
        let mut col = Vec::new();
        for i in 0..nb_rows {
            col.push(lines_2d.arr[i][j]);
        }
        if col.iter().all(|x| *x == '.') {
            empty_cols.push(j as i16);
        }
    }
    // find galaxies
    let mut galaxies = Vec::new();
    for i in 0..nb_rows {
        for j in 0..nb_cols {
            if lines_2d.arr[i][j] == '#' {
                galaxies.push((i as i16, j as i16));
            }
        }
    }

    let mut distance: i64 = 0;
    let mut millions: i64;
    for (i, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(i + 1) {
            millions = 0;
            if a.0 < b.0 {
                millions = empty_rows.iter().fold(0, |acc, x| if (a.0..b.0).contains(x) { acc + 1} else { acc });
            } else if a.0 > b.0 {
                millions = empty_rows.iter().fold(0, |acc, x| if (b.0..a.0).contains(x) { acc + 1} else { acc });
            }
            if a.1 < b.1 {
                millions += empty_cols.iter().fold(0, |acc, x| if (a.1..b.1).contains(x) { acc + 1} else { acc });
            } else {
                millions += empty_cols.iter().fold(0, |acc, x| if (b.1..a.1).contains(x) { acc + 1} else { acc });
            }
            distance += (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs() + millions * (1_000_000 - 1);
        }
    }

    println!("sum of shortest distances: {}", distance);
}
