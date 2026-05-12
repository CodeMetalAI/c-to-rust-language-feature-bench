// ptr_decl.rs
struct Bundle {
    p_to_const: *const i32,
    const_p: *const i32,
    const_p_via_typedef: *const *const i32,
}

fn pick(p: *const i32, bias: i32) -> i32 {
    let v = unsafe { *p };
    if v & 1 != 0 {
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

fn same_addr(x: *const std::ptr::Void, y: *const std::ptr::Void) -> bool {
    x as *const _ == y as *const _
}

fn check_bundle(b: Bundle, expected_const_p_target: *const i32) -> i32 {
    if b.const_p != expected_const_p_target {
        return 100;
    }
    if b.const_p_via_typedef != expected_const_p_target {
        return 101;
    }
    0
}

fn main() {
    let mut x = 10;
    let mut y = 21;

    let ptr_to_constant = &x;

    let constant_ptr = &mut x;
    let clarified_constant_ptr = &mut x;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(b, &mut x) != 0 {
        std::process::exit(1);
    }

    if *b.p_to_const != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    if *b.p_to_const != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    if *b.p_to_const != 21 {
        std::process::exit(4);
    }

    *b.const_p += 5;
    if x != 15 {
        std::process::exit(5);
    }

    *b.const_p_via_typedef += 7;
    if x != 22 {
        std::process::exit(6);
    }

    if check_bundle(b, &mut x) != 0 {
        std::process::exit(7);
    }

    if *b.p_to_const != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}