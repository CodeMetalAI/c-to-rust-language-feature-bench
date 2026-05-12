struct Bundle<'a> {
    p_to_const: &'a i32,
    const_p: &'a mut i32,
    const_p_via_typedef: &'a i32,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const<'a>(b: &mut Bundle<'a>, a: &'a i32, c: &'a i32, sel: i32) {
    if sel != 0 {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr(x: *const (), y: *const ()) -> bool {
    x == y
}

fn check_bundle(b: &Bundle, expected_const_p_target: &i32) -> i32 {
    if !same_addr(b.const_p as *const _ as *const (), expected_const_p_target as *const _) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef as *const _ as *const (), expected_const_p_target as *const _) {
        return 101;
    }
    0
}

fn main() {
    let mut x = 10;
    let mut y = 21;

    let ptr_to_constant: &i32 = &x;

    let constant_ptr: &mut i32 = &mut x;
    let clarified_constant_ptr: &i32 = &x;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &x) != 0 {
        std::process::exit(1);
    }

    if *b.p_to_const != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, if pick(&y, 0) > 0 { 1 } else { 0 });
    if *b.p_to_const != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, if pick(&x, 1) < 0 { 1 } else { 0 });
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

    if check_bundle(&b, &x) != 0 {
        std::process::exit(7);
    }

    if *b.p_to_const != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}