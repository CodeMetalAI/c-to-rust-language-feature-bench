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
    let endless = Rc::new_cyclic(|weak| IntList {
        car: 0,
        cdr: weak.upgrade().unwrap(),
    });
    let result = eval(endless.clone(), &endless);
    std::process::exit(result);
}