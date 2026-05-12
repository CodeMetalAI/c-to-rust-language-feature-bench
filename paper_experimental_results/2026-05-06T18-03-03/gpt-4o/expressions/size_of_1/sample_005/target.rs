fn alloc(n: usize) -> Option<Box<[u8]>> {
    let vec = vec![0u8; n];
    Some(vec.into_boxed_slice())
}

fn main() {
    let dp = alloc(std::mem::size_of::<f64>());

    if dp.is_none() {
        std::process::exit(1);
    }

    let mut dp = dp.unwrap();
    let dp_ptr = dp.as_mut_ptr() as *mut f64;

    unsafe {
        *dp_ptr = 3.14;
        if *dp_ptr != 3.14 {
            std::process::exit(2);
        }
    }

    // Box will be dropped here, automatically freeing the memory
}