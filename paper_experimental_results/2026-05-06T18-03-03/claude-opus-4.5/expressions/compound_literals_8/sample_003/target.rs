fn f() -> bool {
    let mut storage: [Option<(usize, i32)>; 2] = [None, None];
    let mut p: Option<usize> = None;
    let mut q: Option<usize>;
    let mut j = 0;

    loop {
        q = p;
        storage[j] = Some((j, j as i32));
        p = Some(j);
        j += 1;
        if j >= 2 {
            break;
        }
    }

    // Check if p == q (both point to same index) and q->i == 1
    match (p, q) {
        (Some(p_idx), Some(q_idx)) => {
            let same_address = p_idx == q_idx;
            let q_i = storage[q_idx].map(|(_, i)| i).unwrap_or(0);
            same_address && q_i == 1
        }
        _ => false,
    }
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}