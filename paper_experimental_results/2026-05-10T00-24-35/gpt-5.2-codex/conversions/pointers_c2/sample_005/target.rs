use std::rc::Rc;

fn main() {
    let p = Rc::new(17);
    let pc = Rc::clone(&p);
    let pv = Rc::clone(&p);
    let pcv = Rc::clone(&p);

    let code = if !Rc::ptr_eq(&pc, &p) {
        1
    } else if !Rc::ptr_eq(&pv, &p) {
        2
    } else if !Rc::ptr_eq(&pcv, &p) {
        3
    } else if *pc != 17 {
        4
    } else if *pv != 17 {
        5
    } else if *pcv != 17 {
        6
    } else {
        0
    };

    std::process::exit(code);
}