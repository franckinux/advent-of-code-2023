use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::iter::zip;


#[derive(Debug, Eq, Ord, PartialEq)]
enum HandType {
    High(String),
    OnePair(String),
    TwoPair(String),
    Three(String),
    FullHouse(String),
    Four(String),
    Five(String),
}


impl HandType {
    fn from(s: String) -> HandType {
        let mut h: HashMap<char, u16> = HashMap::new();

        for c in s.chars() {
            if let Some(k) = h.get_mut(&c) {
                *k += 1;
            } else {
                h.insert(c, 1);
            }
        }

        let mut v: Vec<&u16> = h.values().collect();
        v.sort_by(|a, b| b.cmp(a));

        match v[..] {
            [5] => HandType::Five(s),
            [4, 1] => HandType::Four(s),
            [3, 2] => HandType::FullHouse(s),
            [3, 1, 1] => HandType::Three(s),
            [2, 2, 1] => HandType::TwoPair(s),
            [2, 1, 1, 1] => HandType::OnePair(s),
            [1, 1, 1, 1, 1] => HandType::High(s),
            _ => panic!("impossible case"),
        }
    }

    fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}


impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        const CARDS: [char; 13] = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

        let d_s = HandType::discriminant(self);
        let d_o = HandType::discriminant(other);
        if d_s == d_o {
            let s_s = match self {
                HandType::High(s) | HandType::OnePair(s) | HandType::TwoPair(s) | HandType::Three(s) | HandType::FullHouse(s) | HandType::Four(s) | HandType::Five(s) => s,
            };
            let s_o = match other {
                HandType::High(s) | HandType::OnePair(s) | HandType::TwoPair(s) | HandType::Three(s) | HandType::FullHouse(s) | HandType::Four(s) | HandType::Five(s) => s,
            };
            let z: Vec<_> = zip(s_s.chars(), s_o.chars()).collect();

            for (s, o) in z {
                let ps = CARDS.iter().position(|r| *r == s).unwrap();
                let po = CARDS.iter().position(|r| *r == o).unwrap();
                if ps < po {
                    return Some(Ordering::Greater);
                }
                if ps > po {
                    return Some(Ordering::Less);
                }
            }
            Some(Ordering::Equal)
        } else {
            if d_s < d_o {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        }
    }
}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_hand(s: &String) -> (HandType, u32) {
    let parts: Vec<&str> = s.split_whitespace().collect();
    (HandType::from(String::from(parts[0])), parts[1].parse().unwrap())
}


fn main() {
    let lines = read_lines("data/day7.txt");

    let mut hands = Vec::new();
    for l in lines {
        hands.push(get_hand(&l));
    }
    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut total = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index as u32 + 1) * hand.1;
    }
    println!("total winning: {}", total);
}
