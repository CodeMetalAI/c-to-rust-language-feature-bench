use std::cell::Cell;
use std::ptr;

#[derive(Copy, Clone)]
struct Bundle<'a> {
    p_to_const: &'a Cell<i32>,
    const_p: &'a Cell<i32>,
    const_p_via_typedef: &'a Cell<i32>,
}

fn pick(p: &Cell<i32>, bias: i32) -> i32 {
    let v = p.get();
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const<'a>(b: &mut Bundle<'a>, a: &'a Cell<i32>, c: &'a Cell<i32>, sel: bool) {
    if sel {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr<T>(x: &T, y: &T) -> bool {
    ptr::eq(x, y)
}

fn check_bundle(b: Bundle<'_>, expected_const_p_target: &Cell<i32>) -> i32 {
    if !same_addr(b.const_p, expected_const_p_target) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef, expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    let x = Cell::new(10);
    let y = Cell::new(21);

    let ptr_to_constant = &x;

    let constant_ptr = &x;
    let clarified_constant_ptr = &x;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(b, &x) != 0 {
        std::process::exit(1);
    }

    if b.p_to_const.get() != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    if b.p_to_const.get() != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    if b.p_to_const.get() != 21 {
        std::process::exit(4);
    }

    let v = b.const_p.get();
    b.const_p.set(v + 5);
    if x.get() != 15 {
        std::process::exit(5);
    }

    let v = b.const_p_via_typedef.get();
    b.const_p_via_typedef.set(v + 7);
    if x.get() != 22 {
        std::process::exit(6);
    }

    if check_bundle(b, &x) != 0 {
        std::process::exit(7);
    }

    if b.p_to_const.get() != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}