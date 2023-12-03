use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn get_game_number(s: String) -> u32 {
    let mut c: char;
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;

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
                'r' => { if number > min_red { min_red = number; }; break; },
                'g' => { if number > min_green { min_green = number; }; break; },
                'b' => { if number > min_blue { min_blue = number; }; break; },
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

    min_red * min_green * min_blue
}


fn main() {
    let lines = read_lines("data/day2.txt");

    let mut power = 0;
    for line in lines.iter() {
        power += get_game_number(line.to_string());
    }

    println!("sum of powers: {}", power);
}
