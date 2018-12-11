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

    const WORKER_COUNT: usize = 5;
    const WORKER_START_TIME: u32 = 60;

    let mut workers = vec![None; WORKER_COUNT];
    let mut output = String::new();
    let mut seconds = 0;
    println!("Second  Worker 1  Worker 2 Done");
    loop {
        while let Some(idle_worker_index) = workers.iter().position(Option::is_none) {
            if keys.is_empty() { break }

            if let Some(key) = get_next_key(&keys, &dependency_graph) {
                keys.retain(|k| k != &key);
                let duration: u32 = WORKER_START_TIME + key as u32 - 'A' as u32 + 1;
                workers[idle_worker_index] = Some((key, duration));
            } else {
                break;
            }
        }
        for i in 0..workers.len() {
            let should_remove = {
                let worker = match workers[i].as_mut() {
                    Some(w) => w,
                    None => {
                        continue;
                    }
                };
                if worker.1 > 0 {
                    worker.1 -= 1;
                }
                if worker.1 == 0 {
                    output.push(worker.0);
                    remove_key_from_graph(worker.0, &mut keys, &mut dependency_graph);
                    true
                } else {
                    false
                }
            };
            if should_remove {
                workers[i] = None;
            }
        }
        seconds += 1;
        if keys.is_empty() && workers.iter().filter(|o| o.is_some()).next().is_none() {
            break;
        }
    }
    println!("{:?}, takes {} seconds", output, seconds);
}

fn get_next_key(keys: &[char], dependency_graph: &HashMap<char, Vec<char>>) -> Option<char> {
    keys.iter()
        .find(|key| dependency_graph[key].is_empty())
        .cloned()
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
