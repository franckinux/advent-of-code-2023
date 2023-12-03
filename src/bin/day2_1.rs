use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_game_number(s: String) -> bool {
    let mut c: char;

    let mut char_iter = s.chars();

    loop {
        if char_iter.next().unwrap() == ':' {
            break;
        }
    }

    'outer: loop {
        loop {
            c = char_iter.next().unwrap();
            if c.is_whitespace() {
                break;
            }
        }

        let mut number = 0;
        loop {
            c = char_iter.next().unwrap();
            if c.is_whitespace() {
                break;
            }
            number = number * 10 + c.to_digit(10).unwrap();
        }

        loop {
            c = char_iter.next().unwrap();
            match c {
                'r' => { if number > 12 { return false }; break; },
                'g' => { if number > 13 { return false }; break; },
                'b' => { if number > 14 { return false }; break; },
                _ => {}
            }
        }

        loop {
            if let Some(c) = char_iter.next() {
                if let ';' | ','  = c {
                    break;
                }
            } else {
                break 'outer;
            }
        }
    }

    true
}


fn main() {
    let lines = read_lines("data/day2.txt");

    let mut sum = 0;
    for (game, line) in lines.iter().enumerate() {
        if get_game_number(line.to_string()) {
            sum += game + 1;
        }
    }

    println!("sum of game ids: {}", sum);
}
