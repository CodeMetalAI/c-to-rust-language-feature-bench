use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct Bundle {
    p_to_const: Rc<RefCell<i32>>,
    const_p: Rc<RefCell<i32>>,
    const_p_via_typedef: Rc<RefCell<i32>>,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: Rc<RefCell<i32>>, c: Rc<RefCell<i32>>, sel: i32) {
    if sel != 0 {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn check_bundle(b: Bundle, expected_const_p_target: Rc<RefCell<i32>>) -> i32 {
    if !Rc::ptr_eq(&b.const_p, &expected_const_p_target) {
        return 100;
    }
    if !Rc::ptr_eq(&b.const_p_via_typedef, &expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    let x = Rc::new(RefCell::new(10));
    let y = Rc::new(RefCell::new(21));

    let mut b = Bundle {
        p_to_const: x.clone(),
        const_p: x.clone(),
        const_p_via_typedef: x.clone(),
    };

    if check_bundle(b.clone(), x.clone()) != 0 {
        std::process::exit(1);
    }

    if *b.p_to_const.borrow() != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, y.clone(), x.clone(), if pick(&*y.borrow(), 0) > 0 { 1 } else { 0 });
    if *b.p_to_const.borrow() != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, x.clone(), y.clone(), if pick(&*x.borrow(), 1) < 0 { 1 } else { 0 });
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

    if check_bundle(b.clone(), x.clone()) != 0 {
        std::process::exit(7);
    }

    if *b.p_to_const.borrow() != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}