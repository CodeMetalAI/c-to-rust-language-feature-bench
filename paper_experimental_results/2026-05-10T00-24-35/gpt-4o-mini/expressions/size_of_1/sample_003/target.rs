fn alloc(size: usize) -> Option<Box<f64>> {
    if size == 0 {
        None
    } else {
        Some(Box::new(0.0))
    }
}

fn main() -> i32 {
    let dp = alloc(std::mem::size_of::<f64>());

    match dp {
        Some(mut value) => {
            *value = 3.14;
            if *value != 3.14 {
                return 2;
            }
        }
        None => return 1,
    }

    // The Box will be dropped here, so we don't need to free it manually.
    0
}