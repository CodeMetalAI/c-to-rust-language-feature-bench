use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
enum StructType {
    S1 { v1: i32, s2p: Option<Rc<RefCell<StructType>>> },
    S2 { v2: i32, s1p: Option<Rc<RefCell<StructType>>> },
}

fn main() {
    let a = Rc::new(RefCell::new(StructType::S1 { v1: 10, s2p: None }));
    let b = Rc::new(RefCell::new(StructType::S2 { v2: 20, s1p: None }));

    // Link them
    if let StructType::S1 { v1, .. } = *a.borrow() {
        *a.borrow_mut() = StructType::S1 { v1, s2p: Some(b.clone()) };
    }
    if let StructType::S2 { v2, .. } = *b.borrow() {
        *b.borrow_mut() = StructType::S2 { v2, s1p: Some(a.clone()) };
    }

    // Checks
    if let StructType::S1 { s2p, .. } = &*a.borrow() {
        if let Some(s2) = s2p {
            if let StructType::S2 { v2, .. } = &*s2.borrow() {
                if *v2 != 20 {
                    std::process::exit(1);
                }
            }
        }
    }

    if let StructType::S2 { s1p, .. } = &*b.borrow() {
        if let Some(s1) = s1p {
            if let StructType::S1 { v1, .. } = &*s1.borrow() {
                if *v1 != 10 {
                    std::process::exit(2);
                }
            }
        }
    }

    if let StructType::S1 { s2p, .. } = &*a.borrow() {
        if let Some(s2) = s2p {
            if let StructType::S2 { s1p, .. } = &*s2.borrow() {
                if let Some(s1) = s1p {
                    if !Rc::ptr_eq(s1, &a) {
                        std::process::exit(3);
                    }
                }
            }
        }
    }

    std::process::exit(0);
}