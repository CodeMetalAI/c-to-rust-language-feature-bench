fn f() -> bool {
    let mut storage: Vec<i32> = Vec::new();
    let mut p: Option<usize> = None;
    let mut q: Option<usize>;
    let mut j = 0;

    loop {
        q = p;
        // Simulate compound literal reusing same storage location
        if storage.is_empty() {
            storage.push(j);
            p = Some(0);
        } else {
            storage[0] = j;
            p = Some(0);
        }
        j += 1;
        
        if j >= 2 {
            break;
        }
    }

    // return (p == q) && (q->i == 1)
    p == q && q.map(|idx| storage[idx] == 1).unwrap_or(false)
}

fn main() {
    std::process::exit(if f() { 0 } else { 1 });
}