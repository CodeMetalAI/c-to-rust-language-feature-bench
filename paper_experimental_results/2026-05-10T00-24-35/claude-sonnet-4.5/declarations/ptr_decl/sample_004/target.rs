struct Bundle<'a> {
    p_to_const: *const i32,
    const_p: *const i32,
    const_p_via_typedef: *const i32,
    _phantom: std::marker::PhantomData<&'a i32>,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: &i32, c: &i32, sel: bool) {
    if sel {
        b.p_to_const = a as *const i32;
    } else {
        b.p_to_const = c as *const i32;
    }
}

fn same_addr(x: *const i32, y: *const i32) -> bool {
    x == y
}

fn check_bundle(b: &Bundle, expected_const_p_target: &i32) -> i32 {
    let expected_ptr = expected_const_p_target as *const i32;
    if !same_addr(b.const_p, expected_ptr) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef, expected_ptr) {
        return 101;
    }
    0
}

fn main() {
    let mut x: i32 = 10;
    let y: i32 = 21;

    let ptr_to_constant: *const i32 = &x as *const i32;
    let constant_ptr: *const i32 = &x as *const i32;
    let clarified_constant_ptr: *const i32 = &x as *const i32;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
        _phantom: std::marker::PhantomData,
    };

    if check_bundle(&b, &x) != 0 {
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

    x += 5;
    if x != 15 {
        std::process::exit(5);
    }

    x += 7;
    if x != 22 {
        std::process::exit(6);
    }

    if check_bundle(&b, &x) != 0 {
        std::process::exit(7);
    }

    if unsafe { *b.p_to_const } != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}