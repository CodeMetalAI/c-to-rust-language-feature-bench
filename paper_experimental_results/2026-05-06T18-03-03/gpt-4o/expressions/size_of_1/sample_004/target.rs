fn alloc(n: usize) -> Option<Box<[u8]>> {
    let vec = vec![0u8; n];
    Some(vec.into_boxed_slice())
}

fn main() {
    let dp = alloc(std::mem::size_of::<f64>()).and_then(|mut boxed_slice| {
        let ptr = boxed_slice.as_mut_ptr() as *mut f64;
        unsafe {
            *ptr = 3.14;
            if *ptr != 3.14 {
                return None;
            }
        }
        Some(boxed_slice)
    });

    if dp.is_none() {
        std::process::exit(1);
    }

    std::process::exit(0);
}