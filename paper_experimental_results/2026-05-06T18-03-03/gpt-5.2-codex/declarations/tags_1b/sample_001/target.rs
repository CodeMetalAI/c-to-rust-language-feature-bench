use std::process::exit;

#[derive(Clone, Copy)]
struct Node {
    count: i32,
    left: Option<usize>,
    right: Option<usize>,
}

fn main() {
    let mut nodes = Vec::new();

    // indices for nodes
    let s = 0usize;
    let left_child = 1usize;
    let right_child = 2usize;

    nodes.push(Node { count: 0, left: None, right: None });
    nodes.push(Node { count: 0, left: None, right: None });
    nodes.push(Node { count: 0, left: None, right: None });

    nodes[s].count = 1;
    nodes[left_child].count = 2;
    nodes[right_child].count = 3;

    nodes[s].left = Some(left_child);
    nodes[s].right = Some(right_child);

    let sp = s;

    if nodes[nodes[sp].left.unwrap()].count != 2 {
        exit(1);
    }

    if nodes[nodes[s].right.unwrap()].count != 3 {
        exit(2);
    }

    if nodes[sp].left == nodes[sp].right {
        exit(3);
    }

    exit(0);
}