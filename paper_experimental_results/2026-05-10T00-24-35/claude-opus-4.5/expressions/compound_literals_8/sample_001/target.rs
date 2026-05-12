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
        if j < 2 {
            continue;
        }
        break;
    }

    // Check if p == q (they point to the same index)
    // and q->i == 1
    if let (Some(p_idx), Some(q_idx)) = (p, q) {
        if p_idx == q_idx {
            if let Some((_, i)) = storage[q_idx] {
                return i == 1;
            }
        }
    }
    false
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}