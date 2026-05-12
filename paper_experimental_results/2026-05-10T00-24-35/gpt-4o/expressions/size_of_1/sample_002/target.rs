fn main() {
    fn alloc(n: usize) -> Option<Box<[u8]>> {
        let vec = vec![0u8; n];
        Some(vec.into_boxed_slice())
    }

    let dp: Option<Box<[u8]>> = alloc(std::mem::size_of::<f64>());

    if dp.is_none() {
        std::process::exit(1);
    }

    let dp = dp.unwrap();
    let mut dp: Box<f64> = unsafe { Box::from_raw(Box::into_raw(dp) as *mut f64) };

    *dp = 3.14;
    if *dp != 3.14 {
        std::process::exit(2);
    }

    // Box will be automatically deallocated when it goes out of scope
}