struct S {
    x: i32,
    y: f64,
}

fn main() -> i32 {
    // Initialize variables
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S { x: 7, y: 9.5 };
    
    // Create raw pointers (safe to create, just can't dereference without unsafe)
    let pi: *const i32 = &xi;
    let pd: *const f64 = &xd;
    let ps: *const S = &xs;
    
    // Create pointer-to-pointer
    let ppi: *const *const i32 = &pi;
    let ppd: *const *const f64 = &pd;
    pps: *const *const S = &ps;

    // Check pointer values (compare addresses)
    // Note: In Rust, comparing raw pointers for equality is safe
    // Pointers should point to their respective variables
    if ppi != &pi {
        return 1;
    }
    if ppd != &pd {
        return 2;
    }
    if pps != &ps {
        return 3;
    }
    
    // Dereference pointers to check values (requires unsafe)
    unsafe {
        if *(*ppi) != 42 {
            return 4;
        }
        if *(*ppd) != 3.25 {
            return 5;
        }
        if (* *pops).x != 7 {
            return 6;
        }
        if (* *pops).y != 9.5 {
            return 7;
        }
    }

    // Everything matched
    0
}