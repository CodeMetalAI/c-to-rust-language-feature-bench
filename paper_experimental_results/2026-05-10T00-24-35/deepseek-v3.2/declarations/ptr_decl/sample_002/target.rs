fn pick(p: *const i32, bias: i32) -> i32 {
    let v = unsafe { *p };
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
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

fn check_bundle(b: &Bundle, expected_const_p_target: *const i32) -> i32 {
    if same_addr(b.const_p.cast(), expected_const_p_target.cast()) == 0 {
        return 100;
    }
    if same_addr(b.const_p_via_typedef.cast(), expected_const_p_target.cast()) == 0 {
        return 101;
    }
    0
}

type IntPtr = *mut i32;

struct Bundle {
    p_to_const: *const i32,
    const_p: *mut i32,
    const_p_via_typedef: IntPtr,
}

fn main() -> i32 {
    let mut x = 10;
    let y = 21;

    let ptr_to_constant: *const i32 = &x;
    let constant_ptr: *mut i32 = &mut x;
    let clarified_constant_ptr: IntPtr = &mut x;

    let b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &x) != 0 {
        return 1;
    }

    if unsafe { *b.p_to_const } != 10 {
        return 2;
    }

    let mut b_mut = b;
    retarget_ptr_to_const(&mut b_mut, &y, &x, pick(&y, 0) > 0);
    if unsafe { *b_mut.p_to_const } != 21 {
        return 3;
    }

    retarget_ptr_to_const(&mut b_mut, &x, &y, pick(&x, 1) < 0);
    if unsafe { *b_mut.p_to_const } != 21 {
        return 4;
    }

    unsafe {
        *b_mut.const_p += 5;
    }
    if x != 15 {
        return 5;
    }

    unsafe {
        *b_mut.const_p_via_typedef += 7;
    }
    if x != 22 {
        return 6;
    }

    if check_bundle(&b_mut, &x) != 0 {
        return 7;
    }

    if unsafe { *b_mut.p_to_const } != 21 {
        return 8;
    }

    0
}