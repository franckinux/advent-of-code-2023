use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_points(s: String) -> i32 {
    let parts: Vec<&str> = s.split(&[':', '|']).collect();
    let wining: Vec<u32> = parts[1].split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();
    let lottery: Vec<u32> = parts[2].split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();

    let points = lottery.iter().fold(0, |acc, x| {
        if let Some(_) = wining.iter().find(|y| x == *y) { acc + 1 } else { acc }
    });

    if points == 0 { 0 } else { 2i32.pow(points - 1) }
}


fn main() {
    let lines = read_lines("data/day4.txt");

    let mut sum = 0;
    for line in lines {
        let points = get_points(line.to_string());
        sum += points;
    }

    println!("sum of points: {}", sum);
}
