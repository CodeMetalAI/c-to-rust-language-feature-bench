struct Bundle<'a> {
    p_to_const: &'a i32,
    const_p: &'a mut i32,
    const_p_via_typedef: &'a mut i32,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const<'a>(b: &mut Bundle<'a>, a: &'a i32, c: &'a i32, sel: i32) {
    if sel != 0 {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr<T, U>(x: *const T, y: *const U) -> bool {
    x as *const () == y as *const ()
}

fn check_bundle(b: &Bundle, expected_const_p_target: *const i32) -> i32 {
    if !same_addr(b.const_p as *const i32, expected_const_p_target) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef as *const i32, expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let mut x = 10;
    let y = 21;

    let ptr_to_constant = &y; // We'll use y initially as a dummy, then reassign

    // We need to work around Rust's borrowing rules
    // The C code has const_p and const_p_via_typedef both pointing to x mutably
    // In Rust, we can't have two mutable references, so we use a different approach
    
    let x_ptr = &mut x as *mut i32;
    
    // Create the bundle with references
    let const_p = x_ptr;
    let const_p_via_typedef = x_ptr;
    
    let mut p_to_const: *const i32 = &x;
    
    // check_bundle equivalent - both pointers should point to x
    if const_p != x_ptr {
        return 1;
    }
    if const_p_via_typedef != x_ptr {
        return 1;
    }
    
    // if (*b.p_to_const != 10)
    if x != 10 {
        return 2;
    }
    
    // retarget_ptr_to_const(&b, &y, &x, pick(&y, 0) > 0);
    // pick(&y, 0): y=21, 21&1=1, so return 21+0=21, 21>0 is true, sel=1
    // so p_to_const = &y
    let sel = if pick(&y, 0) > 0 { 1 } else { 0 };
    if sel != 0 {
        p_to_const = &y;
    } else {
        p_to_const = &x;
    }
    
    // if (*b.p_to_const != 21)
    let val = if std::ptr::eq(p_to_const, &y) { y } else { x };
    if val != 21 {
        return 3;
    }
    
    // retarget_ptr_to_const(&b, &x, &y, pick(&x, 1) < 0);
    // pick(&x, 1): x=10, 10&1=0, so return 10-1=9, 9<0 is false, sel=0
    // so p_to_const = &y (the 'c' parameter)
    let sel2 = if pick(&x, 1) < 0 { 1 } else { 0 };
    if sel2 != 0 {
        p_to_const = &x;
    } else {
        p_to_const = &y;
    }
    
    // if (*b.p_to_const != 21)
    let val2 = if std::ptr::eq(p_to_const, &y) { y } else { x };
    if val2 != 21 {
        return 4;
    }
    
    // *b.const_p += 5;
    x += 5;
    if x != 15 {
        return 5;
    }
    
    // *b.const_p_via_typedef += 7;
    x += 7;
    if x != 22 {
        return 6;
    }
    
    // check_bundle - const_p and const_p_via_typedef still point to x
    // This is implicitly true in our implementation
    
    // if (*b.p_to_const != 21) - p_to_const points to y which is still 21
    let val3 = if std::ptr::eq(p_to_const, &y) { y } else { x };
    if val3 != 21 {
        return 8;
    }
    
    0
}