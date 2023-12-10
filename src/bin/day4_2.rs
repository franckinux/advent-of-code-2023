use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_points(s: String) -> u32 {
    let parts: Vec<&str> = s.split(&[':', '|']).collect();
    let wining: Vec<u32> = parts[1].split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();
    let lottery: Vec<u32> = parts[2].split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();

    lottery.iter().fold(0, |acc, x| {
        if let Some(_) = wining.iter().find(|y| x == *y) { acc + 1 } else { acc }
    })
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
