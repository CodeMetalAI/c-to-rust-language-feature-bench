use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;

type IntList = Rc<RefCell<Node>>;

#[derive(Clone)]
struct Node {
    car: i32,
    cdr: IntList,
}

static ENDLESS_ZEROS: OnceLock<IntList> = OnceLock::new();

fn eval(x: IntList) -> i32 {
    let x_borrow = x.borrow();
    if x_borrow.car != 0 {
        1
    } else if !Rc::ptr_eq(&x_borrow.cdr, ENDLESS_ZEROS.get().unwrap()) {
        2
    } else if !Rc::ptr_eq(&x_borrow.cdr.borrow().cdr, ENDLESS_ZEROS.get().unwrap()) {
        3
    } else {
        0
    }
}

fn main() {
    let endless_zeros = ENDLESS_ZEROS.get_or_init(|| {
        Rc::new_cyclic(|weak| {
            RefCell::new(Node {
                car: 0,
                cdr: weak.upgrade().unwrap(),
            })
        })
    });
    std::process::exit(eval(endless_zeros.clone()));
}