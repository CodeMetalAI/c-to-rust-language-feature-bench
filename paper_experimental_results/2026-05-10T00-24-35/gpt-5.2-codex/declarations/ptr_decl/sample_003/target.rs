use std::cell::RefCell;
use std::process::exit;
use std::rc::Rc;

struct Bundle {
    p_to_const: Rc<RefCell<i32>>,
    const_p: Rc<RefCell<i32>>,
    const_p_via_typedef: Rc<RefCell<i32>>,
}

fn pick(p: &Rc<RefCell<i32>>, bias: i32) -> i32 {
    let v = *p.borrow();
    if (v & 1) != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: &Rc<RefCell<i32>>, c: &Rc<RefCell<i32>>, sel: i32) {
    if sel != 0 {
        b.p_to_const = a.clone();
    } else {
        b.p_to_const = c.clone();
    }
}

fn same_addr(x: &Rc<RefCell<i32>>, y: &Rc<RefCell<i32>>) -> bool {
    Rc::ptr_eq(x, y)
}

fn check_bundle(b: &Bundle, expected_const_p_target: &Rc<RefCell<i32>>) -> i32 {
    if !same_addr(&b.const_p, expected_const_p_target) {
        return 100;
    }
    if !same_addr(&b.const_p_via_typedef, expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    let x = Rc::new(RefCell::new(10));
    let y = Rc::new(RefCell::new(21));

    let ptr_to_constant = x.clone();
    let constant_ptr = x.clone();
    let clarified_constant_ptr = x.clone();

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &x) != 0 {
        exit(1);
    }

    if *b.p_to_const.borrow() != 10 {
        exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, if pick(&y, 0) > 0 { 1 } else { 0 });
    if *b.p_to_const.borrow() != 21 {
        exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, if pick(&x, 1) < 0 { 1 } else { 0 });
    if *b.p_to_const.borrow() != 21 {
        exit(4);
    }

    *b.const_p.borrow_mut() += 5;
    if *x.borrow() != 15 {
        exit(5);
    }

    *b.const_p_via_typedef.borrow_mut() += 7;
    if *x.borrow() != 22 {
        exit(6);
    }

    if check_bundle(&b, &x) != 0 {
        exit(7);
    }

    if *b.p_to_const.borrow() != 21 {
        exit(8);
    }

    exit(0);
}