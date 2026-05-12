fn main() {
    let mut x = 10;
    let y = 21;

    let ptr_to_constant = &x as *const i32;

    let mut constant_ptr = &mut x as *mut i32;
    let clarified_constant_ptr = &mut x as *mut i32;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &mut x) != 0 {
        return std::process::exit(1);
    }

    if unsafe { *b.p_to_const } != 10 {
        return std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    if unsafe { *b.p_to_const } != 21 {
        return std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    if unsafe { *b.p_to_const } != 21 {
        return std::process::exit(4);
    }

    unsafe {
        *b.const_p += 5;
    }
    if x != 15 {
        return std::process::exit(5);
    }

    unsafe {
        *b.const_p_via_typedef += 7;
    }
    if x != 22 {
        return std::process::exit(6);
    }

    if check_bundle(&b, &mut x) != 0 {
        return std::process::exit(7);
    }

    if unsafe { *b.p_to_const } != 21 {
        return std::process::exit(8);
    }
}

struct Bundle {
    p_to_const: *const i32,
    const_p: *mut i32,
    const_p_via_typedef: *mut i32,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: &i32, c: &i32, sel: bool) {
    if sel {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr(x: *const i32, y: *const i32) -> bool {
    x == y
}

fn check_bundle(b: &Bundle, expected_const_p_target: &mut i32) -> i32 {
    if !same_addr(b.const_p as *const _, expected_const_p_target as *const _) {
        100
    } else if !same_addr(b.const_p_via_typedef as *const _, expected_const_p_target as *const _) {
        101
    } else {
        0
    }
}