fn alloc<T>(n: usize) -> Option<T> {
    match Box::leak(Box::new([0; n])) {
        Some(x) => Some(x),
        None => None,
    }
}

fn main() {
    let dp = alloc::<double>();
    if dp.is_none() {
        return;
    }

    let dp = dp.unwrap();
    *dp = 3.14 as double;
    if *dp!= 3.14 {
        return;
    }

    // Note: Rust's garbage collection will handle this automatically.
    // You don't need to call a manual `free` function.
    // The `Box` will be dropped when it goes out of scope, freeing the memory.
}