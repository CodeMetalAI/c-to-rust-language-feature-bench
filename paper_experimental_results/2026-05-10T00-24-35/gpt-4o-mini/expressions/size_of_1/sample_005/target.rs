fn alloc(n: usize) -> Option<Box<f64>> {
    if n == 0 {
        None
    } else {
        Some(Box::new(0.0))
    }
}

fn main() -> i32 {
    let dp = alloc(std::mem::size_of::<f64>());

    if dp.is_none() {
        return 1;
    }

    let mut dp = dp.unwrap();
    *dp = 3.14;

    if *dp != 3.14 {
        return 2;
    }

    // dp will be dropped here, freeing the memory automatically
    0
}