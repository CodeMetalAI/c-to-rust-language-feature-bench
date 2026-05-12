use std::cell::Cell;
use std::rc::Rc;

type IntPtr = Rc<Cell<i32>>;

#[derive(Clone)]
struct Bundle {
    p_to_const: IntPtr,
    const_p: IntPtr,
    const_p_via_typedef: IntPtr,
}

fn pick(p: &IntPtr, bias: i32) -> i32 {
    let v = p.get();
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: &IntPtr, c: &IntPtr, sel: i32) {
    if sel != 0 {
        b.p_to_const = a.clone();
    } else {
        b.p_to_const = c.clone();
    }
}

fn same_addr(x: &IntPtr, y: &IntPtr) -> bool {
    Rc::ptr_eq(x, y)
}

fn check_bundle(b: &Bundle, expected_const_p_target: &IntPtr) -> i32 {
    if !same_addr(&b.const_p, expected_const_p_target) {
        return 100;
    }
    if !same_addr(&b.const_p_via_typedef, expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    let x: IntPtr = Rc::new(Cell::new(10));
    let y: IntPtr = Rc::new(Cell::new(21));

    let ptr_to_constant = x.clone();

    let constant_ptr = x.clone();
    let clarified_constant_ptr = x.clone();

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

    let sel = if pick(&y, 0) > 0 { 1 } else { 0 };
    retarget_ptr_to_const(&mut b, &y, &x, sel);
    if b.p_to_const.get() != 21 {
        std::process::exit(3);
    }

    let sel = if pick(&x, 1) < 0 { 1 } else { 0 };
    retarget_ptr_to_const(&mut b, &x, &y, sel);
    if b.p_to_const.get() != 21 {
        std::process::exit(4);
    }

    b.const_p.set(b.const_p.get() + 5);
    if x.get() != 15 {
        std::process::exit(5);
    }

    b.const_p_via_typedef
        .set(b.const_p_via_typedef.get() + 7);
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