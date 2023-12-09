use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_points(s: String) -> i32 {
    let mut c: char;

    let mut char_iter = s.chars();

    loop {
        if char_iter.next().unwrap() == ':' {
            break;
        }
    }

    let mut number = 0;
    let mut winning = Vec::new();
    loop {
        c = char_iter.next().unwrap();
        if c.is_whitespace() {
            if number != 0 {
                winning.push(number);
            }
            number = 0;
            continue;
        } else if c == '|' {
            break;
        }
        number = number * 10 + c.to_digit(10).unwrap();
    }

    let mut number = 0;
    let mut points = 0;
    let mut end = false;
    loop {
        match char_iter.next() {
            Some(ch) => { c = ch; },
            None => { end = true },
        }
        if end || c.is_whitespace() {
            if number != 0 {
                if winning.iter().find(|&n| *n == number).is_some() {
                    points += 1;
                }
            }
            if end { break; }
            number = 0;
        } else {
            number = number * 10 + c.to_digit(10).unwrap();
        }
    }

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
