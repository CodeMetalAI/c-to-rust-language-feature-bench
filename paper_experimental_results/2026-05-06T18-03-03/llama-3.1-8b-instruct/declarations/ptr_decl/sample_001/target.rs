// ptr_decl.rs
struct Bundle {
    p_to_const: *const i32,
    const_p: *const i32,
    const_p_via_typedef: *const i32,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(bundle: &mut Bundle, a: &i32, c: &i32, sel: bool) {
    if sel {
        bundle.p_to_const = a as *const i32;
    } else {
        bundle.p_to_const = c as *const i32;
    }
}

fn same_addr(x: &dyn Any, y: &dyn Any) -> bool {
    std::ptr::eq(x as *const _, y as *const _)
}

fn check_bundle(bundle: Bundle, expected_const_p_target: &i32) -> i32 {
    if !same_addr(bundle.const_p, expected_const_p_target) {
        100
    } else if !same_addr(bundle.const_p_via_typedef, expected_const_p_target) {
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

    if check_bundle(b, &x) != 0 {
        return 1;
    }

    if *b.p_to_const != 10 {
        return 2;
    }

    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    if *b.p_to_const != 21 {
        return 3;
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    if *b.p_to_const != 21 {
        return 4;
    }

    unsafe {
        *b.const_p = 15;
    }
    if x != 15 {
        return 5;
    }

    unsafe {
        *b.const_p_via_typedef = 22;
    }
    if x != 22 {
        return 6;
    }

    if check_bundle(b, &x) != 0 {
        return 7;
    }

    if *b.p_to_const != 21 {
        return 8;
    }

    return 0;
}