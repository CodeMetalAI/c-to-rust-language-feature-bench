use std::rc::Rc;

fn main() -> i32 {
    let x = Rc::new(17);
    let p = Rc::clone(&x);
    let pc = Rc::clone(&x);
    let pv = Rc::clone(&x);
    let pcv = Rc::clone(&x);

    if !Rc::ptr_eq(&pc, &p) {
        return 1;
    }
    if !Rc::ptr_eq(&pv, &p) {
        return 2;
    }
    if !Rc::ptr_eq(&pcv, &p) {
        return 3;
    }

    if *pc != 17 {
        return 4;
    }
    if *pv != 17 {
        return 5;
    }
    if *pcv != 17 {
        return 6;
    }

    0
}