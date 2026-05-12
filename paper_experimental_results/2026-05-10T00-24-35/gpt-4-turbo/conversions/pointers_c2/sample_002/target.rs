fn main() {
    let x = 17;
    let p = &x;
    
    let pc: &i32 = p;
    let pv: &i32 = p;
    let pcv: &i32 = p;
    
    // In Rust, all these pointers point to the same location
    // Rust doesn't have a direct equivalent to the volatile keyword in the same sense as C++
    // but for this example, the checks can remain purely conceptual, as Rust ensures these are identical.
    
    if pc as *const i32 != p as *const i32 {
        std::process::exit(1);
    }
    if pv as *const i32 != p as *const i32 {
        std::process::exit(2);
    }
    if pcv as *const i32 != p as *const i32 {
        std::process::exit(3);
    }
    
    // Check values via dereferencing
    if *pc != 17 {
        std::process::exit(4);
    }
    if *pv != 17 {
        std::process::exit(5);
    }
    if *pcv != 17 {
        std::process::exit(6);
    }
}