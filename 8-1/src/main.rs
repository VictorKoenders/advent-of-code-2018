use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut iter = str.split(' ').map(|s| s.trim().parse().unwrap());
    let root_node = Node::read(&mut iter);

    println!("{:#?}", root_node);
    println!("Sum of metadata is {}", root_node.get_metadata_sum());
}

#[derive(Debug)]
pub struct Node {
    pub child_nodes: Vec<Node>,
    pub metadata_entries: Vec<u32>,
}

impl Node {
    pub fn read(it: &mut Iterator<Item = u32>) -> Node {
        let children_count = it.next().unwrap();
        let metadata_entries_count = it.next().unwrap();

        let mut child_nodes = Vec::with_capacity(children_count as usize);
        let mut metadata_entries = Vec::with_capacity(metadata_entries_count as usize);
        for _ in 0..children_count {
            child_nodes.push(Node::read(it));
        }
        for _ in 0..metadata_entries_count {
            metadata_entries.push(it.next().unwrap());
        }

        Node {
            child_nodes,
            metadata_entries
        }
    }

    pub fn get_metadata_sum(&self) -> u32 {
        let mut result = 0;
        for entry in &self.metadata_entries {
            result += *entry;
        }
        for child in &self.child_nodes {
            result += child.get_metadata_sum();
        }
        result
    }
}

