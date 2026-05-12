fn alloc(n: usize) -> Option<Box<[u8]>> {
    Some(vec![0u8; n].into_boxed_slice())
}

fn main() {
    let mut dp = alloc(std::mem::size_of::<f64>()).map(|mut boxed| {
        unsafe { std::mem::transmute::<Box<[u8]>, Box<f64>>(boxed) }
    });

    if dp.is_none() {
        std::process::exit(1);
    }

    let dp = dp.as_mut().unwrap();
    *dp = 3.14;

    if *dp != 3.14 {
        std::process::exit(2);
    }

    std::process::exit(0);
}