use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_calibration_value(s: String) -> u32 {
    const FIGURES: [(&str, u32); 18] = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut figure_tens = 0;
    let mut indexes = Vec::new();
    for (figure_str, figure_num) in FIGURES {
        if let Some(index) = s.find(figure_str) {
            indexes.push((index, figure_num))
        } else {
            continue;
        }
    }
    let mut min_index = 1_000_000;
    for (i, f) in indexes {
        if i <= min_index {
            min_index = i;
            figure_tens = f;
        }
    }

    let mut figure_unit = 0;
    let mut indexes = Vec::new();
    for (figure_str, figure_num) in FIGURES {
        if let Some(index) = s.rfind(figure_str) {
            indexes.push((index, figure_num))
        } else {
            continue;
        }
    }
    let mut max_index = 0;
    for (i, f) in indexes {
        if i  >= max_index {
            max_index = i;
            figure_unit = f;
        }
    }

    figure_tens * 10 + figure_unit
}


fn main() {
    let lines = read_lines("data/day1.txt");

    let mut sum = 0u32;
    for line in lines {
        sum += get_calibration_value(line.to_string());
    }

    println!("sum of calibration values: {}", sum);
}
