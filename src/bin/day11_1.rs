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
    let mut nb_rows = lines.len();
    let mut nb_cols = lines[0].len();

    let mut lines_2d = Array2D::new(nb_rows, nb_cols, ' ');
    for r in 0..nb_rows {
        for c in 0..nb_cols {
            lines_2d.set(r, c, lines[r][c]).unwrap();
        }
    }

    // expand the universe
    for i in (0..nb_rows).rev() {
        if lines_2d.arr[i].as_slice().iter().all(|x| *x == '.') {
            lines_2d.arr.insert(i, vec!['.'; nb_cols]);
            nb_rows += 1;
        }
    }
    for j in (0..nb_cols).rev() {
        let mut col = Vec::new();
        for i in 0..nb_rows {
            col.push(lines_2d.arr[i][j]);
        }
        if col.iter().all(|x| *x == '.') {
            for i in 0..nb_rows {
                lines_2d.arr[i].insert(j, '.');
            }
            nb_cols += 1;
        }
    }
    lines_2d.cols = nb_cols;
    lines_2d.rows = nb_rows;

    // find galaxies
    let mut galaxies = Vec::new();
    for i in 0..nb_rows {
        for j in 0..nb_cols {
            if lines_2d.arr[i][j] == '#' {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    let mut distance = 0;
    for (i, a) in galaxies.iter().enumerate() {
        for b in galaxies.iter().skip(i + 1) {
            distance += (a.0 - b.0).abs() + (a.1 - b.1).abs();
        }
    }

    println!("sum of shortest distances: {}", distance);
}
