use std::process::exit;

#[derive(Clone)]
struct Node {
    count: i32,
    left: usize,
    right: usize,
}

fn main() {
    let left_child = Node {
        count: 2,
        left: 0,
        right: 0,
    };
    let right_child = Node {
        count: 3,
        left: 0,
        right: 0,
    };

    let mut nodes = Vec::new();
    let left_idx = nodes.len();
    nodes.push(left_child);
    let right_idx = nodes.len();
    nodes.push(right_child);

    let s = Node {
        count: 1,
        left: left_idx,
        right: right_idx,
    };
    let s_idx = nodes.len();
    nodes.push(s);

    let sp = s_idx;

    if nodes[nodes[sp].left].count != 2 {
        exit(1);
    }

    if nodes[nodes[s_idx].right].count != 3 {
        exit(2);
    }

    if nodes[sp].left == nodes[sp].right {
        exit(3);
    }

    exit(0);
}