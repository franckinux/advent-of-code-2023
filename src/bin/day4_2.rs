use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_points(s: String) -> u32 {
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

    points
}


fn main() {
    #[derive(Debug)]
    struct Card {
        nbr_of_wins: u32,
        nbr_of_cards: u32,
    }

    let lines = read_lines("data/day4.txt");
    let mut cards: Vec<Card> = Vec::new();

    for line in lines {
        cards.push(
            Card {
                nbr_of_wins: get_points(line.to_string()),
                nbr_of_cards: 1,
            }
        );
    }

    for index1 in 0..cards.len() {
        for index2 in 1..=cards[index1].nbr_of_wins {
            cards[index1 + index2 as usize].nbr_of_cards += cards[index1].nbr_of_cards;
        }
    }

    let sum = cards.iter().fold(0, |acc, x| acc + x.nbr_of_cards);
    println!("sum of cards: {}", sum);
}
