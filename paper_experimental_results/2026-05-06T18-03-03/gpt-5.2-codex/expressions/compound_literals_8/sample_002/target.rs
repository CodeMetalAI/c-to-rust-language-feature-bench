use std::rc::Rc;

struct S {
    i: i32,
}

fn f() -> bool {
    let mut p: Option<Rc<S>> = None;
    let mut q: Option<Rc<S>> = None;
    let mut j: i32 = 0;

    loop {
        q = p.clone();
        let new_obj = Rc::new(S { i: j });
        j += 1;
        p = Some(new_obj);

        if j >= 2 {
            break;
        }
    }

    let p_ref = match p {
        Some(ref r) => r,
        None => return false,
    };
    let q_ref = match q {
        Some(ref r) => r,
        None => return false,
    };

    Rc::ptr_eq(p_ref, q_ref) && q_ref.i == 1
}

fn main() {
    let code = if f() { 0 } else { 1 };
    std::process::exit(code);
}