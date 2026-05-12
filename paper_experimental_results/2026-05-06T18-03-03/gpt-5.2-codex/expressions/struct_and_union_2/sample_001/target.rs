use std::process::exit;

#[derive(Clone, Copy)]
struct S {
    i: i32,
    ci: i32,
}

fn f(_p: &i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &i32) {}
fn g(_p: &i32) {}

fn main() {
    let mut s = S { i: 1, ci: 2 };
    let cs = S { i: 3, ci: 4 };
    let mut vs = S { i: 5, ci: 6 };

    f(&s.i);
    f4(&s.ci);

    f4(&cs.i);
    f4(&cs.ci);

    f2(&vs.i);
    g(&vs.ci);

    if s.i != 1 {
        exit(1);
    }
    if s.ci != 2 {
        exit(2);
    }
    if cs.i != 3 {
        exit(3);
    }
    if cs.ci != 4 {
        exit(4);
    }
    if vs.i != 5 {
        exit(5);
    }
    if vs.ci != 6 {
        exit(6);
    }

    s.i = 10;
    if s.i != 10 {
        exit(7);
    }

    vs.i = 20;
    if vs.i != 20 {
        exit(8);
    }

    exit(0);
}