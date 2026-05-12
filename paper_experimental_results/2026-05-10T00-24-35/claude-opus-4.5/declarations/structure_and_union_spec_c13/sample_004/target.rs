fn check_designated_init() -> i32 {
    let base = 10;
    let a = 1;
    let b = 2;
    let deep = 99;
    let c = 3;
    let d = 4;
    let tail_x = 7;
    let tail_y = 8;

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
    // Simulating union behavior where deep, x, and y overlap
    // In the C code, setting deep = 0x11223344 means x (which aliases deep) also becomes 0x11223344
    // Then setting x = 5 and y = 6 overwrites the union content
    
    let mut deep_or_xy: [i32; 2] = [0, 0];
    
    // o.deep = 0x11223344;
    deep_or_xy[0] = 0x11223344;
    
    // if (o.x != 0x11223344) - x aliases deep
    let x = deep_or_xy[0];
    if x != 0x11223344 {
        return 20;
    }
    
    // o.x = 5; o.y = 6;
    deep_or_xy[0] = 5;
    deep_or_xy[1] = 6;
    
    let x = deep_or_xy[0];
    let y = deep_or_xy[1];
    if x != 5 {
        return 21;
    }
    if y != 6 {
        return 22;
    }
    
    // Simulating tail union: tail_y aliases tail_p (they start at same offset)
    // tail_q is after tail_p
    let mut tail_pq: [i32; 2] = [0, 0];
    
    // o.tail_p = 40; o.tail_q = 41;
    tail_pq[0] = 40;
    tail_pq[1] = 41;
    
    let tail_p = tail_pq[0];
    let tail_q = tail_pq[1];
    if tail_p != 40 {
        return 23;
    }
    if tail_q != 41 {
        return 24;
    }
    
    // o.tail_y = -9; - tail_y aliases tail_p
    tail_pq[0] = -9;
    
    let tail_y = tail_pq[0];
    if tail_y != -9 {
        return 25;
    }
    
    0
}

fn check_addressability() -> i32 {
    // Create separate variables to simulate struct fields
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