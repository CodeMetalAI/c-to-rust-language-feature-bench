fn main() {
    let x = 10;
    let y = 21;

    let ptr_to_constant = &x;

    let constant_ptr = &x;
    let clarified_constant_ptr = constant_ptr;

    let b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(b.clone(), &x) != 0 {
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

    if check_bundle(b.clone(), &x) != 0 {
        return 7;
    }

    if *b.p_to_const != 21 {
        return 8;
    }

    println!("Success!");
}

struct Bundle {
    p_to_const: *const i32,
    const_p: *const i32,
    const_p_via_typedef: *const i32,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 != 0 {
        return v + bias;
    }
    return v - bias;
}

fn retarget_ptr_to_const(b: &mut Bundle, a: &i32, c: &i32, sel: bool) {
    if sel {
        b.p_to_const = a as *const i32;
    } else {
        b.p_to_const = c as *const i32;
    }
}

fn same_addr(x: *const (), y: *const ()) -> bool {
    x == y
}

fn check_bundle(b: Bundle, expected_const_p_target: &i32) -> i32 {
    if !same_addr(b.const_p, expected_const_p_target) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef, expected_const_p_target) {
        return 101;
    }
    0
}