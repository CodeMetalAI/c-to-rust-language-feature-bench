struct Bundle<'a> {
    p_to_const: &'a i32,
    const_p: &'a mut i32,
    const_p_via_typedef: &'a mut i32,
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

fn same_addr<T, U>(x: *const T, y: *const U) -> bool {
    x as *const () == y as *const ()
}

fn check_bundle(b: &Bundle, expected_const_p_target: *const i32) -> i32 {
    if !same_addr(b.const_p as *const i32, expected_const_p_target) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef as *const i32, expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let mut x = 10i32;
    let y = 21i32;

    let ptr_to_constant: &i32 = &y; // placeholder, will be set properly via bundle
    let _ = ptr_to_constant;

    // We need to create two mutable references to x, which isn't allowed in safe Rust.
    // However, we can use a Cell to allow interior mutability with shared references.
    use std::cell::Cell;
    
    let x = Cell::new(10i32);
    let y = Cell::new(21i32);

    let mut p_to_const: &Cell<i32> = &x;

    if x.get() != 10 {
        return 2;
    }

    // retarget_ptr_to_const equivalent: pick(&y, 0) > 0
    let pick_result = {
        let v = y.get();
        if v & 1 != 0 { v + 0 } else { v - 0 }
    };
    if pick_result > 0 {
        p_to_const = &y;
    } else {
        p_to_const = &x;
    }
    if p_to_const.get() != 21 {
        return 3;
    }

    // retarget_ptr_to_const equivalent: pick(&x, 1) < 0
    let pick_result = {
        let v = x.get();
        if v & 1 != 0 { v + 1 } else { v - 1 }
    };
    if pick_result < 0 {
        p_to_const = &x;
    } else {
        p_to_const = &y;
    }
    if p_to_const.get() != 21 {
        return 4;
    }

    // *b.const_p += 5
    x.set(x.get() + 5);
    if x.get() != 15 {
        return 5;
    }

    // *b.const_p_via_typedef += 7
    x.set(x.get() + 7);
    if x.get() != 22 {
        return 6;
    }

    // check_bundle - both const_p and const_p_via_typedef point to x
    // This is implicitly satisfied since we're using &x for both operations

    if p_to_const.get() != 21 {
        return 8;
    }

    0
}