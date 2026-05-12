use std::rc::Rc;
use std::process;

struct Tnode {
    count: i32,
    left: Option<Rc<Tnode>>,
    right: Option<Rc<Tnode>>,
}

fn main() {
    let left_child = Rc::new(Tnode { count: 2, left: None, right: None });
    let right_child = Rc::new(Tnode { count: 3, left: None, right: None });
    let s_rc = Rc::new(Tnode {
        count: 1,
        left: Some(Rc::clone(&left_child)),
        right: Some(Rc::clone(&right_child)),
    });
    let sp = Rc::clone(&s_rc);
    if sp.left.as_ref().unwrap().count != 2 {
        process::exit(1);
    }
    if s_rc.right.as_ref().unwrap().count != 3 {
        process::exit(2);
    }
    if Rc::ptr_eq(&sp.left.as_ref().unwrap(), &sp.right.as_ref().unwrap()) {
        process::exit(3);
    }
    process::exit(0);
}