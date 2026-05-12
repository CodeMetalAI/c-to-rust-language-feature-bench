fn alloc(n: usize) -> Option<Box<f64>> {
    if n == std::mem::size_of::<f64>() {
        Some(Box::new(0.0))
    } else {
        None
    }
}

fn main() {
    let dp = alloc(std::mem::size_of::<f64>());

    let mut dp = match dp {
        Some(p) => p,
        None => std::process::exit(1),
    };

    *dp = 3.14;
    if *dp != 3.14 {
        std::process::exit(2);
    }

    std::process::exit(0);
}