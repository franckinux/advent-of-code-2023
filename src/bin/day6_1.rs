use std::fs::read_to_string;
use std::iter::zip;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_values(s: &String) -> Vec<u32> {
    let parts: Vec<String> = s.split(':').map(String::from).collect();
    parts[1].split_whitespace().map(|x| x.parse().unwrap()).collect()
}


fn main() {
    let lines = read_lines("data/day6.txt");

    let mut line_iter = lines.iter();

    let times = get_values(line_iter.next().unwrap());
    let distances = get_values(line_iter.next().unwrap());

    let mut result = 1;
    for (time, distance) in zip(times, distances) {
        let mut records = 0;
        for t in 1..time {
            let d = (time - t) * t;
            if d > distance {
                records += 1;
            }
        }
        result *= records;
    }

    println!("record: {}", result);
}
