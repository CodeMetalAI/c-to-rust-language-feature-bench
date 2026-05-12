use std::rc::Rc;

struct IntList {
    car: i32,
    cdr: Rc<IntList>,
}

fn eval(x: Rc<IntList>, endless: &Rc<IntList>) -> i32 {
    if x.car != 0 {
        return 1;
    }
    if !Rc::ptr_eq(&x.cdr, endless) {
        return 2;
    }
    if !Rc::ptr_eq(&x.cdr.cdr, endless) {
        return 3;
    }
    0
}

fn main() {
    let endless_zeros = Rc::new_cyclic(|self_ref| IntList {
        car: 0,
        cdr: self_ref.upgrade().unwrap(),
    });

    let result = eval(endless_zeros.clone(), &endless_zeros);
    std::process::exit(result);
}