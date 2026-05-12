use std::rc::Rc;
use std::cell::RefCell;

struct Bundle {
    p_to_const: Rc<RefCell<i32>>,
    const_p: Rc<RefCell<i32>>,
    const_p_via_typedef: Rc<RefCell<i32>>,
}

impl Clone for Bundle {
    fn clone(&self) -> Self {
        Bundle {
            p_to_const: self.p_to_const.clone(),
            const_p: self.const_p.clone(),
            const_p_via_typedef: self.const_p_via_typedef.clone(),
        }
    }
}

fn pick(p: &Rc<RefCell<i32>>, bias: i32) -> i32 {
    let v = *p.borrow();
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: Rc<RefCell<i32>>, c: Rc<RefCell<i32>>, sel: bool) {
    if sel {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn check_bundle(b: Bundle, expected: Rc<RefCell<i32>>) -> i32 {
    if !Rc::ptr_eq(&b.const_p, &expected) {
        return 100;
    }
    if !Rc::ptr_eq(&b.const_p_via_typedef, &expected) {
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

    if check_bundle(b.clone(), x.clone()) != 0 {
        std::process::exit(1);
    }

    if *b.p_to_const.borrow() != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, y.clone(), x.clone(), pick(&y, 0) > 0);
    if *b.p_to_const.borrow() != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, x.clone(), y.clone(), pick(&x, 1) < 0);
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