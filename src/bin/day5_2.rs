use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_seeds(s: &String) -> Vec<Vec<u64>> {
    let parts: Vec<&str> = s.split(':').collect();
    let items: Vec<u64> = parts[1].split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();

    let mut item_iter = items.iter();
    let mut seeds: Vec<Vec<u64>> = Vec::new();
    loop {
        if let Some(a) = item_iter.next() {
            let b = item_iter.next().unwrap();
            seeds.push(vec![*a, *b]);
        } else {
            break;
        }
    }

    seeds
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

    let seeds_ranges = get_seeds(line_iter.next().unwrap());

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
    for sr in seeds_ranges {
        for s in sr[0]..sr[0] + sr[1] {
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
    }

    println!("lowest location number: {}", location);
}
