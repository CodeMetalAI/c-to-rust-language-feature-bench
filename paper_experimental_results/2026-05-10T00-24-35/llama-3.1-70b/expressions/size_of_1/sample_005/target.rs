fn main() {
    let dp = std::alloc::alloc(std::mem::size_of::<double>());

    if dp.is_null() {
        std::process::exit(1);
    }

    let dp = dp as *mut double;
    unsafe { *dp = 3.14; }

    if unsafe { *dp }!= 3.14 {
        std::alloc::dealloc(dp as *mut _, std::mem::size_of::<double>(), std::alloc::Layout::from_size_align(std::mem::size_of::<double>(), std::mem::align_of::<double>()).unwrap());
        std::process::exit(2);
    }

    std::alloc::dealloc(dp as *mut _, std::mem::size_of::<double>(), std::alloc::Layout::from_size_align(std::mem::size_of::<double>(), std::mem::align_of::<double>()).unwrap());
    std::process::exit(0);
}