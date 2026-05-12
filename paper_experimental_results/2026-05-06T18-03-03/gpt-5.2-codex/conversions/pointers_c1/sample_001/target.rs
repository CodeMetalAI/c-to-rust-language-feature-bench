use std::ptr;

struct S {
    a: i32,
    b: f64,
}

enum VoidPtr<'a> {
    Int(&'a i32),
    Double(&'a f64),
    S(&'a S),
}

impl<'a> PartialEq for VoidPtr<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VoidPtr::Int(a), VoidPtr::Int(b)) => ptr::eq(*a, *b),
            (VoidPtr::Double(a), VoidPtr::Double(b)) => ptr::eq(*a, *b),
            (VoidPtr::S(a), VoidPtr::S(b)) => ptr::eq(*a, *b),
            _ => false,
        }
    }
}
impl<'a> Eq for VoidPtr<'a> {}

fn main() {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi = &xi;
    let pd = &xd;
    let ps = &xs;

    let v1 = VoidPtr::Int(pi);
    let v2 = VoidPtr::Double(pd);
    let v3 = VoidPtr::S(ps);

    let pi2 = match &v1 {
        VoidPtr::Int(r) => *r,
        _ => std::process::exit(1),
    };
    let pd2 = match &v2 {
        VoidPtr::Double(r) => *r,
        _ => std::process::exit(2),
    };
    let ps2 = match &v3 {
        VoidPtr::S(r) => *r,
        _ => std::process::exit(3),
    };

    if !ptr::eq(pi2, pi) {
        std::process::exit(1);
    }
    if !ptr::eq(pd2, pd) {
        std::process::exit(2);
    }
    if !ptr::eq(ps2, ps) {
        std::process::exit(3);
    }

    if *pi2 != 42 {
        std::process::exit(4);
    }
    if *pd2 != 3.25 {
        std::process::exit(5);
    }
    if ps2.a != 7 {
        std::process::exit(6);
    }
    if ps2.b != 9.5 {
        std::process::exit(7);
    }

    let v1b = VoidPtr::Int(pi2);
    let v2b = VoidPtr::Double(pd2);
    let v3b = VoidPtr::S(ps2);

    if v1b != v1 {
        std::process::exit(8);
    }
    if v2b != v2 {
        std::process::exit(9);
    }
    if v3b != v3 {
        std::process::exit(10);
    }
}