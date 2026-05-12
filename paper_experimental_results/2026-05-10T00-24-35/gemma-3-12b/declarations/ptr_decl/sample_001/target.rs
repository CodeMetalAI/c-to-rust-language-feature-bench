#[derive(Debug)]
struct bundle {
    p_to_const: *const i32,
    const_p: *mut i32,
    const_p_via_typedef: *const i32,
}

fn pick(p: *const i32, bias: i32) -> i32 {
    let v = unsafe { *p };
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut bundle, a: *const i32, c: *const i32, sel: bool) {
    if sel {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr(x: *const std::os::raw::c::c_void, y: *const std::os::raw::c::c_void) -> i32 {
    if x == y {
        1
    } else {
        0
    }
}

fn check_bundle(b: bundle, expected_const_p_target: *mut i32) -> i32 {
    if same_addr(
        b.const_p as *const std::os::raw::c::c_void,
        expected_const_p_target as *const std::os::raw::c::c_void,
    ) == 0
    {
        100
    } else if same_addr(
        b.const_p_via_typedef as *const std::os::raw::c::c_void,
        expected_const_p_target as *const std::os::raw::c::c_void,
    ) == 0
    {
        101
    } else {
        0
    }
}

fn main() {
    let mut x = 10;
    let mut y = 21;

    let ptr_to_constant = &x;

    let constant_ptr = &mut x;
    let clarified_constant_ptr = &x;

    let mut b = bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(b, &mut x) != 0 {
        std::process::exit(1);
    }

    if unsafe { *b.p_to_const } != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    if unsafe { *b.p_to_const } != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    if unsafe { *b.p_to_const } != 21 {
        std::process::exit(4);
    }

    unsafe { *b.const_p += 5 };
    if x != 15 {
        std::process::exit(5);
    }

    unsafe { *b.const_p_via_typedef += 7 };
    if x != 22 {
        std::process::exit(6);
    }

    if check_bundle(b, &mut x) != 0 {
        std::process::exit(7);
    }

    if unsafe { *b.p_to_const } != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}