use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_seeds(s: &String) -> Vec<u64> {
    let sp: Vec<&str> = s.split(':').collect();
    sp[1].split_whitespace().map(|x| x.to_string().parse().unwrap()).collect()
}


fn get_destination(val: u64, map: &Vec<u64>) -> Option<u64> {
    if val >= map[1] && val < map[1] + map[2] {
        Some(val + map[0] - map[1])
    } else {
        None
    }
}


fn main() {
    let lines = read_lines("data/day5.txt");

    let mut line_iter = lines.iter();

    let seeds = get_seeds(line_iter.next().unwrap());

    let mut ranges: Vec<Vec<Vec<u64>>> = Vec::new();
    'outer: loop {
        let mut maps: Vec<Vec<u64>> = Vec::new();
        loop {
            if let Some(l) = line_iter.next() {
                if let Some(c) = l.chars().nth(0) {
                    if c.is_digit(10) {
                        let map: Vec<u64> = l.split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();
                        maps.push(map.clone());
                    }
                } else {
                    break;
                }
            } else {
                ranges.push(maps.clone());
                break 'outer;
            }
        }
        if maps.len() != 0 {
            ranges.push(maps.clone());
        }
    }

    let mut location = 1_000_000_000_000;
    for s in seeds {
        let mut v = s;
        for r in &ranges {
            for ms in r {
                if let Some(d) = get_destination(v, ms) {
                    v = d;
                    break;
                }
            }
        }

        if v < location {
            location = v;
        }
    }

    println!("lowest location number: {}", location);
}
