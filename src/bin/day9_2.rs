use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn main() {
    let lines = read_lines("data/day9.txt");

    let mut result: i64 = 0;
    for l in lines.iter() {
        let mut firsts: Vec<i64> = Vec::new();
        let mut step: Vec<i64> = l.split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();
        firsts.push(*step.iter().next().unwrap());
        loop {
            let mut step_new: Vec<i64> = Vec::new();
            let mut step_iter = step.iter();
            let mut v = step_iter.next().unwrap();
            for w in step_iter {
                step_new.push(w - v);
                v = w;
            }
            if step_new.iter().all(|x| *x == 0) {
                break;
            }
            firsts.push(*step_new.iter().next().unwrap());
            step = step_new;
        }
        let mut sign = -1;
        let missing = firsts.iter().fold(0, |acc, x| {sign *= -1; acc + x * sign});
        result += missing;
    }

    println!("sum of extrapolated values: {}", result) ;
}
