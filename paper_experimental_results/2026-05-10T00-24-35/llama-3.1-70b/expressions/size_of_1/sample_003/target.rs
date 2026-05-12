fn main() {
    let dp = std::alloc::alloc(std::mem::size_of::<double>());

    if dp.is_null() {
        std::process::exit(1);
    }

    let dp = dp as *mut double;
    unsafe { *dp = 3.14; }

    if unsafe { *dp }!= 3.14 {
        std::alloc::dealloc(dp, std::mem::size_of::<double>());
        std::process::exit(2);
    }

    std::alloc::dealloc(dp, std::mem::size_of::<double>());
    std::process::exit(0);
}