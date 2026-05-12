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

fn same_addr(x: *const i32, y: *const i32) -> bool {
    x == y
}

fn check_bundle(b: &Bundle, expected_const_p_target: &i32) -> i32 {
    if !same_addr(b.const_p as *const _, expected_const_p_target as *const _) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef as *const _, expected_const_p_target as *const _) {
        return 101;
    }
    0
}

fn main() -> i32 {
    let mut x = 10;
    let mut y = 21;

    let ptr_to_constant = &x;

    let constant_ptr: &mut i32 = &mut x;
    let clarified_constant_ptr: &i32 = &x;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &x) != 0 {
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

    *b.const_p += 5;
    if x != 15 {
        return 5;
    }

    *b.const_p_via_typedef += 7;
    if x != 22 {
        return 6;
    }

    if check_bundle(&b, &x) != 0 {
        return 7;
    }

    if *b.p_to_const != 21 {
        return 8;
    }

    0
}