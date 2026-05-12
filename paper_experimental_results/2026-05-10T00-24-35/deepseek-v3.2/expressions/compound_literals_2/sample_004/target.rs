fn f(init: i32) -> i32 {
    let x = init;
    let mut p: &[i32; 2] = &[x, 0];
    
    // In C, this compound literal creates a new array with the first element copied from *p
    // and the second element zero-initialized, then assigns the pointer to that array.
    // In Rust, we can't directly replicate the compound literal behavior, but we can
    // create a new array with the same semantics.
    let arr = [p[0], 0];
    p = &arr;
    
    if p[0] != init {
        return 1;
    }
    if p[1] != 0 {
        return 2;
    }
    
    0
}

fn main() {
    if f(7) != 0 {
        std::process::exit(10);
    }
    if f(-3) != 0 {
        std::process::exit(11);
    }
}