use std::rc::Rc;

#[derive(Clone)]
struct IntList {
    car: i32,
    cdr: Rc<IntList>,
}

thread_local! {
    static ENDLESS: Rc<IntList> = Rc::new_cyclic(|weak| IntList {
        car: 0,
        cdr: weak.upgrade().unwrap(),
    });
}

fn endless() -> Rc<IntList> {
    ENDLESS.with(|rc| rc.clone())
}

fn eval(x: IntList) -> i32 {
    if x.car != 0 {
        return 1;
    }
    let endless_ref = endless();
    if !Rc::ptr_eq(&x.cdr, &endless_ref) {
        return 2;
    }
    if !Rc::ptr_eq(&x.cdr.as_ref().cdr, &endless_ref) {
        return 3;
    }
    0
}

fn main() {
    let e = endless();
    let x = (*e).clone();
    let res = eval(x);
    std::process::exit(res);
}