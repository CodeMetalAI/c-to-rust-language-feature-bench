use std::rc::Rc;

fn main() {
    let x = Rc::new(17);
    let p = x.clone();
    let pc = x.clone();
    let pv = x.clone();
    let pcv = x.clone();

    if !Rc::ptr_eq(&p, &pc) {
        std::process::exit(1);
    }
    if !Rc::ptr_eq(&p, &pv) {
        std::process::exit(2);
    }
    if !Rc::ptr_eq(&p, &pcv) {
        std::process::exit(3);
    }

    if *pc != 17 {
        std::process::exit(4);
    }
    if *pv != 17 {
        std::process::exit(5);
    }
    if *pcv != 17 {
        std::process::exit(6);
    }

    std::process::exit(0);
}