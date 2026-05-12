// ptr_decl.rs
struct Bundle {
    p_to_const: *const i32,
    const_p: *const i32,
    const_p_via_typedef: *const i32,
}

fn pick(p: *const i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1!= 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: *const i32, c: *const i32, sel: bool) {
    if sel {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr(x: *const (), y: *const ()) -> bool {
    x as *const () == y as *const ()
}

fn check_bundle(b: Bundle, expected_const_p_target: *const i32) -> i32 {
    if!same_addr(b.const_p as *const (), expected_const_p_target as *const ()) {
        100
    } else if!same_addr(b.const_p_via_typedef as *const (), expected_const_p_target as *const ()) {
        101
    } else {
        0
    }
}

fn main() {
    let x = 10;
    let y = 21;

    let ptr_to_constant = &x as *const i32;

    let constant_ptr = &x as *const i32;
    let clarified_constant_ptr = &x as *const i32;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(b, &x as *const i32)!= 0 {
        return 1;
    }

    if *b.p_to_const!= 10 {
        return 2;
    }

    retarget_ptr_to_const(&mut b, &y as *const i32, &x as *const i32, pick(&y as *const i32, 0) > 0);
    if *b.p_to_const!= 21 {
        return 3;
    }

    retarget_ptr_to_const(&mut b, &x as *const i32, &y as *const i32, pick(&x as *const i32, 1) < 0);
    if *b.p_to_const!= 21 {
        return 4;
    }

    unsafe {
        *b.const_p += 5;
    }
    if x!= 15 {
        return 5;
    }

    unsafe {
        *b.const_p_via_typedef += 7;
    }
    if x!= 22 {
        return 6;
    }

    if check_bundle(b, &x as *const i32)!= 0 {
        return 7;
    }

    if *b.p_to_const!= 21 {
        return 8;
    }

    return 0;
}