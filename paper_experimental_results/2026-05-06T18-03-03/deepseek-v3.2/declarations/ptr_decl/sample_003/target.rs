type IntPtr = *mut i32;

struct Bundle {
    p_to_const: *const i32,
    const_p: *mut i32,
    const_p_via_typedef: IntPtr,
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

fn same_addr(x: *const std::ffi::c_void, y: *const std::ffi::c_void) -> i32 {
    (x == y) as i32
}

fn check_bundle(b: &Bundle, expected_const_p_target: *mut i32) -> i32 {
    if same_addr(b.const_p as *const _, expected_const_p_target as *const _) == 0 {
        return 100;
    }
    if same_addr(b.const_p_via_typedef as *const _, expected_const_p_target as *const _) == 0 {
        return 101;
    }
    0
}

fn main() {
    let mut x = 10;
    let mut y = 21;

    let ptr_to_constant: *const i32 = &x;

    let constant_ptr: *mut i32 = &mut x;
    let clarified_constant_ptr: IntPtr = &mut x;

    let b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &mut x) != 0 {
        std::process::exit(1);
    }

    unsafe {
        if *b.p_to_const != 10 {
            std::process::exit(2);
        }
    }

    let mut b = b;
    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    unsafe {
        if *b.p_to_const != 21 {
            std::process::exit(3);
        }
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    unsafe {
        if *b.p_to_const != 21 {
            std::process::exit(4);
        }
    }

    unsafe {
        *b.const_p += 5;
        if x != 15 {
            std::process::exit(5);
        }

        *b.const_p_via_typedef += 7;
        if x != 22 {
            std::process::exit(6);
        }
    }

    if check_bundle(&b, &mut x) != 0 {
        std::process::exit(7);
    }

    unsafe {
        if *b.p_to_const != 21 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}