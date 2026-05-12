fn alloc(n: usize) -> *mut f64 {
    let layout = std::alloc::Layout::from_size_align(n, std::mem::align_of::<f64>()).unwrap();
    unsafe { std::alloc::alloc(layout) as *mut f64 }
}

fn main() {
    let dp = alloc(std::mem::size_of::<f64>());

    if dp.is_null() {
        std::process::exit(1);
    }

    unsafe {
        *dp = 3.14;
        if *dp != 3.14 {
            let layout = std::alloc::Layout::from_size_align(
                std::mem::size_of::<f64>(),
                std::mem::align_of::<f64>(),
            )
            .unwrap();
            std::alloc::dealloc(dp as *mut u8, layout);
            std::process::exit(2);
        }

        let layout = std::alloc::Layout::from_size_align(
            std::mem::size_of::<f64>(),
            std::mem::align_of::<f64>(),
        )
        .unwrap();
        std::alloc::dealloc(dp as *mut u8, layout);
    }

    std::process::exit(0);
}