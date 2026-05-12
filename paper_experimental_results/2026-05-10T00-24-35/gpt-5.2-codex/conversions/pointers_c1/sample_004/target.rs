use std::process::exit;

#[derive(Clone, Copy)]
struct S {
    a: i32,
    b: f64,
}

#[derive(Clone, Copy)]
enum VoidPtr<'a> {
    Int(&'a i32),
    Double(&'a f64),
    Struct(&'a S),
}

impl<'a> PartialEq for VoidPtr<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VoidPtr::Int(a), VoidPtr::Int(b)) => std::ptr::eq(*a, *b),
            (VoidPtr::Double(a), VoidPtr::Double(b)) => std::ptr::eq(*a, *b),
            (VoidPtr::Struct(a), VoidPtr::Struct(b)) => std::ptr::eq(*a, *b),
            _ => false,
        }
    }
}

fn main() {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi = &xi;
    let pd = &xd;
    let ps = &xs;

    let v1 = VoidPtr::Int(pi);
    let v2 = VoidPtr::Double(pd);
    let v3 = VoidPtr::Struct(ps);

    let pi2 = match v1 {
        VoidPtr::Int(r) => r,
        _ => exit(1),
    };
    let pd2 = match v2 {
        VoidPtr::Double(r) => r,
        _ => exit(2),
    };
    let ps2 = match v3 {
        VoidPtr::Struct(r) => r,
        _ => exit(3),
    };

    if !std::ptr::eq(pi2, pi) {
        exit(1);
    }
    if !std::ptr::eq(pd2, pd) {
        exit(2);
    }
    if !std::ptr::eq(ps2, ps) {
        exit(3);
    }

    if *pi2 != 42 {
        exit(4);
    }
    if *pd2 != 3.25 {
        exit(5);
    }
    if ps2.a != 7 {
        exit(6);
    }
    if ps2.b != 9.5 {
        exit(7);
    }

    let v1b = VoidPtr::Int(pi2);
    let v2b = VoidPtr::Double(pd2);
    let v3b = VoidPtr::Struct(ps2);

    if v1b != v1 {
        exit(8);
    }
    if v2b != v2 {
        exit(9);
    }
    if v3b != v3 {
        exit(10);
    }

    exit(0);
}