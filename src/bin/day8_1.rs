use std::collections::HashMap;
use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn get_node(mut s: String) -> (String, (String, String)) {
    s.retain(|c| !c.is_whitespace());
    let parts: Vec<&str> = s.split(&['=', ',', '(', ')']).collect();
    (parts[0].to_string(), (parts[2].to_string(), parts[3].to_string()))
}


fn main() {
    let lines = read_lines("data/day8.txt");

    let mut iter_lines = lines.iter();

    let steps = iter_lines.next().unwrap();
    iter_lines.next().unwrap();

    let mut h: HashMap<String, (String, String)> = HashMap::new();
    for line in iter_lines {
        let node = get_node(line.to_string());
        h.insert(node.0, node.1);
    }

    let mut steps_number = 0;
    let mut node_name = "AAA".to_string();
    let mut iter_steps = steps.chars().cycle();
    loop {
        steps_number += 1;
        if let Some(dir) = iter_steps.next() {
            let nodes = h.get(&node_name).unwrap();
            if dir == 'L' {
                node_name = nodes.0.clone();
            } else {
                node_name = nodes.1.clone();
            };
            if node_name == "ZZZ" {
                break;
            };
        }
    }

    println!("number of steps: {}", steps_number) ;
}
