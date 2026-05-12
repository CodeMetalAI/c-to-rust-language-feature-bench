#[derive(Copy, Clone)]
enum VoidPtr<'a> {
    Null,
    Data(&'a i32),
    Func(fn(i32) -> i32),
}

impl<'a> PartialEq for VoidPtr<'a> {
    fn eq(&self, other: &Self) -> bool {
        use VoidPtr::*;
        match (self, other) {
            (Null, Null) => true,
            (Data(a), Data(b)) => std::ptr::eq(*a, *b),
            (Func(a), Func(b)) => a == b,
            _ => false,
        }
    }
}

fn f(x: i32) -> i32 {
    x + 1
}

fn void_to_int_ptr<'a>(v: VoidPtr<'a>) -> Option<&'a i32> {
    match v {
        VoidPtr::Data(r) => Some(r),
        _ => None,
    }
}

fn void_to_func_ptr(v: VoidPtr<'_>) -> Option<fn(i32) -> i32> {
    match v {
        VoidPtr::Func(fp) => Some(fp),
        _ => None,
    }
}

fn main() {
    let x: i32 = 1;
    let px: &i32 = &x;
    let pf: fn(i32) -> i32 = f;

    let n1: Option<&i32> = None;
    let v0: VoidPtr = VoidPtr::Null;
    let n2: Option<&i32> = void_to_int_ptr(v0);

    let nf1: Option<fn(i32) -> i32> = None;
    let vf0: VoidPtr = VoidPtr::Null;
    let nf2: Option<fn(i32) -> i32> = void_to_func_ptr(vf0);

    if n1.is_some() {
        std::process::exit(1);
    }
    if n2.is_some() {
        std::process::exit(2);
    }
    if v0 != VoidPtr::Null {
        std::process::exit(3);
    }

    if n1 == Some(px) {
        std::process::exit(4);
    }
    if n2 == Some(px) {
        std::process::exit(5);
    }

    if nf1.is_some() {
        std::process::exit(6);
    }
    if nf2.is_some() {
        std::process::exit(7);
    }
    if nf1 == Some(pf) {
        std::process::exit(8);
    }
    if nf2 == Some(pf) {
        std::process::exit(9);
    }

    let vx: VoidPtr = VoidPtr::Data(px);
    let vf: VoidPtr = VoidPtr::Func(pf);

    if vx == VoidPtr::Null {
        std::process::exit(10);
    }
    if vf == VoidPtr::Null {
        std::process::exit(11);
    }
    if vx == v0 {
        std::process::exit(12);
    }
    if vf == vf0 {
        std::process::exit(13);
    }

    std::process::exit(0);
}