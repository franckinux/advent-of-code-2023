use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_value(s: &String) -> u64 {
    let mut parts: Vec<String> = s.split(':').map(String::from).collect();
    parts[1].retain(|c| c.is_digit(10));
    parts[1].parse().unwrap()
}


fn main() {
    let lines = read_lines("data/day6.txt");

    let mut line_iter = lines.iter();

    let time = get_value(line_iter.next().unwrap());
    let distance = get_value(line_iter.next().unwrap());

    let mut records = 0;
    for t in 1..time {
        let d = (time - t) * t;
        if d > distance {
            records += 1;
        }
    }

    println!("record: {}", records);
}
