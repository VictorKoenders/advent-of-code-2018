use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut dependency_graph = HashMap::new();
    for line in str.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut chars = line.chars();
        let depends_on = chars.nth(5).unwrap();
        let step = chars.nth(30).unwrap();

        dependency_graph
            .entry(step)
            .or_insert_with(Vec::new)
            .push(depends_on);
        dependency_graph.entry(depends_on).or_insert_with(Vec::new);
    }

    println!("{:?}", dependency_graph);

    let mut keys: Vec<char> = dependency_graph.keys().cloned().collect();
    keys.sort();
    while !keys.is_empty() {
        let key = get_next_key(&keys, &dependency_graph);
        print!("{}", key);
        remove_key_from_graph(key, &mut keys, &mut dependency_graph);
    }
    println!();
}

fn get_next_key(keys: &[char], dependency_graph: &HashMap<char, Vec<char>>) -> char {
    keys.iter()
        .find(|key| dependency_graph[key].is_empty())
        .unwrap()
        .clone()
}

fn remove_key_from_graph(
    key: char,
    keys: &mut Vec<char>,
    dependency_graph: &mut HashMap<char, Vec<char>>,
) {
    for value in dependency_graph.values_mut() {
        value.retain(|v| *v != key);
    }
    dependency_graph.remove(&key);
    keys.retain(|k| *k != key);
}
