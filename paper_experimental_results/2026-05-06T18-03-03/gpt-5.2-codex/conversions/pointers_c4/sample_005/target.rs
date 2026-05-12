use std::process::exit;

fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: Option<usize> = None;
    let v0: Option<usize> = None;
    let p1: Option<usize> = v0;

    let fp0: Option<fn(i32) -> i32> = None;
    let fp1: Option<fn(i32) -> i32> = None;
    let fp2: Option<fn(i32) -> i32> = None;

    if p0 != p1 {
        exit(1);
    }
    if p0 != None {
        exit(2);
    }

    if v0 != None {
        exit(3);
    }
    if v0 != p0 {
        exit(4);
    }

    if fp0 != fp1 {
        exit(5);
    }
    if fp1 != fp2 {
        exit(6);
    }
    if fp0 != None {
        exit(7);
    }

    if p0.is_none() != fp0.is_none() {
        exit(8);
    }

    exit(0);
}