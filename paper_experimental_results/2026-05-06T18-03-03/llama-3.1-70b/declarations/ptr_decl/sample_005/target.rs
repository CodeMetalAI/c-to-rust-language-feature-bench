fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 == 1 {
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

fn same_addr(x: &i32, y: &i32) -> bool {
    x as *const i32 == y as *const i32
}

fn check_bundle(b: &Bundle, expected_const_p_target: &i32) -> i32 {
    if !same_addr(b.const_p, expected_const_p_target) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef, expected_const_p_target) {
        return 101;
    }
    0
}

#[derive(Debug, Default)]
struct Bundle {
    p_to_const: *const i32,
    const_p: *mut i32,
    const_p_via_typedef: *mut i32,
}

fn main() {
    let mut x = 10;
    let mut y = 21;

    let ptr_to_constant = &x as *const i32;

    let constant_ptr = &mut x as *mut i32;
    let clarified_constant_ptr = &mut x as *mut i32;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &x) != 0 {
        return;
    }

    if unsafe { *b.p_to_const } != 10 {
        return;
    }

    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    if unsafe { *b.p_to_const } != 21 {
        return;
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    if unsafe { *b.p_to_const } != 21 {
        return;
    }

    unsafe {
        *b.const_p += 5;
    }
    if x != 15 {
        return;
    }

    unsafe {
        *b.const_p_via_typedef += 7;
    }
    if x != 22 {
        return;
    }

    if check_bundle(&b, &x) != 0 {
        return;
    }

    if unsafe { *b.p_to_const } != 21 {
        return;
    }
}