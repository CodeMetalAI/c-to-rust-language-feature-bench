use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
struct Bundle {
    p_to_const: Rc<Cell<i32>>,
    const_p: Rc<Cell<i32>>,
    const_p_via_typedef: Rc<Cell<i32>>,
}

fn pick(p: &Rc<Cell<i32>>, bias: i32) -> i32 {
    let v = p.get();
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: &Rc<Cell<i32>>, c: &Rc<Cell<i32>>, sel: bool) {
    if sel {
        b.p_to_const = Rc::clone(a);
    } else {
        b.p_to_const = Rc::clone(c);
    }
}

fn same_addr(x: &Rc<Cell<i32>>, y: &Rc<Cell<i32>>) -> bool {
    Rc::ptr_eq(x, y)
}

fn check_bundle(b: &Bundle, expected_const_p_target: &Rc<Cell<i32>>) -> i32 {
    if !same_addr(&b.const_p, expected_const_p_target) {
        return 100;
    }
    if !same_addr(&b.const_p_via_typedef, expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    let x = Rc::new(Cell::new(10));
    let y = Rc::new(Cell::new(21));

    let ptr_to_constant = Rc::clone(&x);
    let constant_ptr = Rc::clone(&x);
    let clarified_constant_ptr = Rc::clone(&x);

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &x) != 0 {
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

    if check_bundle(&b, &x) != 0 {
        std::process::exit(7);
    }

    if b.p_to_const.get() != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}