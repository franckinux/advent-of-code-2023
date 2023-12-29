use std::fs::read_to_string;
use std::collections::HashMap;


struct Array2D<T> {
    arr: Vec<Vec<T>>,
    cols: usize,
    rows: usize,
}

impl<T: Copy> Array2D<T> {
    fn new(r: usize, c: usize, v: T) -> Self {
        Array2D { arr: vec![vec![v; c]; r], cols: c, rows: r }
    }

    fn get(&self, r: i16, c: i16) -> Option<T> {
        if r < 0 || c < 0 {
            return None;
        }
        if let Some(r) = self.arr.get(r as usize) {
            if let Some(v) = r.get(c as usize) {
                return Some(*v);
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
    let lines = read_lines("data/day10.txt");
    let nb_rows = lines.len();
    let nb_cols = lines[0].len();

    let mut lines_2d = Array2D::new(nb_rows, nb_cols, ' ');
    for r in 0..nb_rows {
        for c in 0..nb_cols {
            lines_2d.set(r, c, lines[r][c]).unwrap();
        }
    }

    // look for starting position
    let mut prev_pos = (0, 0);
    'outer: for i in 0..nb_rows {
        for j in 0..nb_cols {
            if lines[i][j] == 'S'{
                prev_pos = (i as i16, j as i16);
                break 'outer;
            }
        }
    }

    let mut sides: HashMap<char, [(i16, i16); 2]> = HashMap::new();
    sides.insert('-', [(0, 1), (0, -1)]);
    sides.insert('|', [(-1, 0), (1, 0)]);
    sides.insert('L', [(-1, 0), (0, 1)]);
    sides.insert('J', [(-1, 0), (0, -1)]);
    sides.insert('7', [(1, 0), (0, -1)]);
    sides.insert('F', [(0, 1), (1, 0) ]);

    const AROUND: [(i16, i16); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut pos = (0, 0);
    for (v, h) in AROUND.iter() {
        pos = (prev_pos.0 + v, prev_pos.1 + h);
        if let Some(ch) = lines_2d.get(pos.0, pos.1) {
            if let Some(t2) = sides.get(&ch) {
                if t2.contains(&(-v, -h)) {
                    break;
                }
            }
        }
    }

    let mut length = 1;
    'outer: loop {
        let ch = lines_2d.get(pos.0, pos.1).unwrap();
        let mut new_pos;
        for (v, h) in sides.get(&ch).unwrap() {
            new_pos = (pos.0 + v, pos.1 + h);
            if let Some(ch) = lines_2d.get(new_pos.0, new_pos.1) {
                if new_pos != prev_pos {
                    length += 1;
                    if ch == 'S' {
                        break 'outer;
                    } else if ch != '.' {
                        prev_pos = pos;
                        pos = new_pos;
                        break;
                    }
                }
            }
        }
    }

    println!("farthest distance: {}", length / 2);
}
