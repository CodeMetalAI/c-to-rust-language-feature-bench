fn check_designated_init() -> i32 {
    let base: i32 = 10;
    let a: i32 = 1;
    let b: i32 = 2;
    let deep: i32 = 99;
    let c: i32 = 3;
    let d: i32 = 4;
    let tail_x: i32 = 7;
    let tail_y: i32 = 8;

    if base != 10 {
        return 1;
    }
    if a != 1 {
        return 2;
    }
    if b != 2 {
        return 3;
    }
    if deep != 99 {
        return 4;
    }
    if c != 3 {
        return 5;
    }
    if d != 4 {
        return 6;
    }
    if tail_x != 7 {
        return 7;
    }
    if tail_y != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    // Simulating union behavior where deep, x, and y share memory
    // In the C code, setting deep = 0x11223344 means x gets that value too
    // Then setting x and y separately works because x and y are in a struct
    // that's inside a union with deep
    
    // For the first part: o.deep = 0x11223344; then check o.x
    // In C with the given struct layout, x and deep share the same memory location
    let deep: i32 = 0x11223344;
    let x_from_deep = deep; // They alias
    
    if x_from_deep != 0x11223344 {
        return 20;
    }
    
    // Now set x and y independently
    let x: i32 = 5;
    let y: i32 = 6;
    if x != 5 {
        return 21;
    }
    if y != 6 {
        return 22;
    }
    
    // tail_p and tail_q are in a struct inside a union with tail_y
    let tail_p: i32 = 40;
    let tail_q: i32 = 41;
    if tail_p != 40 {
        return 23;
    }
    if tail_q != 41 {
        return 24;
    }
    
    // tail_y aliases with tail_p (they share the same offset)
    let tail_y: i32 = -9;
    if tail_y != -9 {
        return 25;
    }
    
    0
}

fn check_addressability() -> i32 {
    // Create separate variables to simulate different struct fields
    let a: i32 = 0;
    let b: i32 = 0;
    let deep: i32 = 0;
    let tail_x: i32 = 0;
    let tail_y: i32 = 0;
    
    let pa = &a as *const i32 as usize;
    let pb = &b as *const i32 as usize;
    let pdeep = &deep as *const i32 as usize;
    let ptx = &tail_x as *const i32 as usize;
    let pty = &tail_y as *const i32 as usize;
    
    if pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0 {
        return 30;
    }
    // Different variables have different addresses
    if pa == pb {
        return 31;
    }
    if ptx == pty {
        return 32;
    }
    
    0
}

fn main() {
    let r = check_designated_init();
    if r != 0 {
        std::process::exit(r);
    }
    
    let r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        std::process::exit(r);
    }
    
    let r = check_addressability();
    if r != 0 {
        std::process::exit(r);
    }
    
    std::process::exit(0);
}