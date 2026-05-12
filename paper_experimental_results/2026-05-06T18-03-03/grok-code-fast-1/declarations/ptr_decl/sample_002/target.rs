use std::cell::RefCell;

type IntPtr = *mut i32;

struct Bundle<'a> {
    p_to_const: &'a RefCell<i32>,
    const_p: &'a RefCell<i32>,
    const_p_via_typedef: &'a RefCell<i32>,
}

fn pick(p: &RefCell<i32>, bias: i32) -> i32 {
    let v = *p.borrow();
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: &RefCell<i32>, c: &RefCell<i32>, sel: i32) {
    if sel != 0 {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr(x: *const (), y: *const ()) -> i32 {
    if x == y { 1 } else { 0 }
}

fn check_bundle(b: Bundle, expected_const_p_target: &RefCell<i32>) -> i32 {
    if same_addr(b.const_p as *const (), expected_const_p_target as *const ()) == 0 {
        return 100;
    }
    if same_addr(b.const_p_via_typedef as *const (), expected_const_p_target as *const ()) == 0 {
        return 101;
    }
    0
}

fn main() {
    let x = RefCell::new(10);
    let y = RefCell::new(21);

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

    if *b.p_to_const.borrow() != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, if pick(&y, 0) > 0 { 1 } else { 0 });
    if *b.p_to_const.borrow() != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, if pick(&x, 1) < 0 { 1 } else { 0 });
    if *b.p_to_const.borrow() != 21 {
        std::process::exit(4);
    }

    *b.const_p.borrow_mut() += 5;
    if *x.borrow() != 15 {
        std::process::exit(5);
    }

    *b.const_p_via_typedef.borrow_mut() += 7;
    if *x.borrow() != 22 {
        std::process::exit(6);
    }

    if check_bundle(b, &x) != 0 {
        std::process::exit(7);
    }

    if *b.p_to_const.borrow() != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}