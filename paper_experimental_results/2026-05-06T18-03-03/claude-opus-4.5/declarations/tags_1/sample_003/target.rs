struct Tnode {
    count: i32,
    left: Option<usize>,
    right: Option<usize>,
}

fn main() {
    let mut nodes: Vec<Tnode> = Vec::new();
    
    // Create left_child at index 0
    nodes.push(Tnode {
        count: 2,
        left: None,
        right: None,
    });
    let left_child_idx = 0;
    
    // Create right_child at index 1
    nodes.push(Tnode {
        count: 3,
        left: None,
        right: None,
    });
    let right_child_idx = 1;
    
    // Create s at index 2
    nodes.push(Tnode {
        count: 1,
        left: Some(left_child_idx),
        right: Some(right_child_idx),
    });
    let s_idx = 2;
    
    let sp_idx = s_idx;
    
    // if (sp->left->count != 2) return 1;
    if let Some(left_idx) = nodes[sp_idx].left {
        if nodes[left_idx].count != 2 {
            std::process::exit(1);
        }
    }
    
    // if (s.right->count != 3) return 2;
    if let Some(right_idx) = nodes[s_idx].right {
        if nodes[right_idx].count != 3 {
            std::process::exit(2);
        }
    }
    
    // if (sp->left == sp->right) return 3;
    if nodes[sp_idx].left == nodes[sp_idx].right {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}