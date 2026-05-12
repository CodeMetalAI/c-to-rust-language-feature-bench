use std::rc::Rc;

#[derive(Clone)]
struct IntList {
    car: i32,
    cdr: Rc<IntList>,
}

fn main() -> i32 {
    let endless_zeros = Rc::new_cyclic(|weak| IntList {
        car: 0,
        cdr: weak.upgrade().unwrap(),
    });

    fn eval(x: Rc<IntList>, endless: &Rc<IntList>) -> i32 {
        if x.car != 0 {
            1
        } else if !Rc::ptr_eq(&x.cdr, endless) {
            2
        } else if !Rc::ptr_eq(&x.cdr.cdr, endless) {
            3
        } else {
            0
        }
    }

    eval(endless_zeros.clone(), &endless_zeros)
}