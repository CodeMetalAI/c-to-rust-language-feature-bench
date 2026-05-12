struct Bundle {
    p_to_const: &mut &i32,
    const_p: &mut i32,
    const_p_via_typedef: &mut i32,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: &i32, c: &i32, sel: i32) {
    *b.p_to_const = if sel != 0 { a } else { c };
}

fn same_addr(x: *const (), y: *const ()) -> i32 {
    if x == y { 1 } else { 0 }
}

fn check_bundle(b: &Bundle, expected_const_p_target: &mut i32) -> i32 {
    let bp = b.const_p as *const i32 as *const ();
    let ep = expected_const_p_target as *const i32 as *const ();
    if same_addr(bp, ep) == 0 {
        return 100;
    }
    let bpt = b.const_p_via_typedef as *const i32 as *const ();
    if same_addr(bpt, ep) == 0 {
        return 101;
    }
    0
}

fn main() {
    let mut x = 10;
    let y = 21;

    let ptr_to_constant = &x;

    let constant_ptr = &mut x;
    let clarified_constant_ptr = &mut x;

    let mut p_to_const_ref = ptr_to_constant;
    let mut b = Bundle {
        p_to_const: &mut p_to_const_ref,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &mut x) != 0 {
        std::process::exit(1);
    }

    if **b.p_to_const != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    if **b.p_to_const != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    if **b.p_to_const != 21 {
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

    if check_bundle(&b, &mut x) != 0 {
        std::process::exit(7);
    }

    if **b.p_to_const != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}