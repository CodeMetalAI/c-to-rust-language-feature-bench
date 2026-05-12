struct Bundle {
    p_to_const: *const i32,
    const_p: *mut i32,
    const_p_via_typedef: *mut i32,
}

fn pick(p: *const i32, bias: i32) -> i32 {
    unsafe {
        let v = *p;
        if v & 1 != 0 {
            v + bias
        } else {
            v - bias
        }
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: *const i32, c: *const i32, sel: i32) {
    if sel != 0 {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr(x: *const (), y: *const ()) -> i32 {
    if x == y { 1 } else { 0 }
}

fn check_bundle(b: Bundle, expected_const_p_target: *const i32) -> i32 {
    if same_addr(b.const_p as *const (), expected_const_p_target as *const ()) == 0 {
        return 100;
    }
    if same_addr(b.const_p_via_typedef as *const (), expected_const_p_target as *const ()) == 0 {
        return 101;
    }
    0
}

fn main() {
    let mut x: i32 = 10;
    let mut y: i32 = 21;

    let ptr_to_constant: *const i32 = &x as *const i32;

    let constant_ptr: *mut i32 = &mut x as *mut i32;
    let clarified_constant_ptr: *mut i32 = &mut x as *mut i32;

    let b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(b, &x as *const i32) != 0 {
        std::process::exit(1);
    }

    unsafe {
        if *b.p_to_const != 10 {
            std::process::exit(2);
        }
    }

    let mut b = b;
    retarget_ptr_to_const(
        &mut b,
        &y as *const i32,
        &x as *const i32,
        if pick(&y as *const i32, 0) > 0 { 1 } else { 0 },
    );
    unsafe {
        if *b.p_to_const != 21 {
            std::process::exit(3);
        }
    }

    retarget_ptr_to_const(
        &mut b,
        &x as *const i32,
        &y as *const i32,
        if pick(&x as *const i32, 1) < 0 { 1 } else { 0 },
    );
    unsafe {
        if *b.p_to_const != 21 {
            std::process::exit(4);
        }
    }

    unsafe {
        *b.const_p += 5;
    }
    if x != 15 {
        std::process::exit(5);
    }

    unsafe {
        *b.const_p_via_typedef += 7;
    }
    if x != 22 {
        std::process::exit(6);
    }

    if check_bundle(b, &x as *const i32) != 0 {
        std::process::exit(7);
    }

    unsafe {
        if *b.p_to_const != 21 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}