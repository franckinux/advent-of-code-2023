use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_calibration_value(s: String) -> u32 {
    let mut c: char;
    #[allow(unused_assignments)]
    let mut number = 0;

    let mut char_iter = s.chars();
    loop {
        c = char_iter.next().unwrap();
        if c.is_digit(10) {
            number = c.to_digit(10).unwrap();
            break;
        }
    }

    let mut char_iter = s.chars().rev();
    loop {
        c = char_iter.next().unwrap();
        if c.is_digit(10) {
            number = number * 10 + c.to_digit(10).unwrap();
            break;
        }
    }

    number
}


fn main() {
    let lines = read_lines("data/day1.txt");

    let mut sum = 0u32;
    for line in lines {
        sum += get_calibration_value(line.to_string());
    }

    println!("sum of calibration values: {}", sum);
}
