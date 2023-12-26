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
        let mut lasts: Vec<i64> = Vec::new();
        let mut step: Vec<i64> = l.split_whitespace().map(|x| x.to_string().parse().unwrap()).collect();
        lasts.push(*step.iter().last().unwrap());
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
            lasts.push(*step_new.iter().last().unwrap());
            step = step_new;
        }
        let missing = lasts.iter().sum::<i64>();
        result += missing;
    }

    println!("sum of extrapolated values: {}", result) ;
}
