use std::cell::RefCell;
use std::process::exit;

#[derive(Clone, Copy)]
struct S {
    i: i32,
    ci: i32,
}

thread_local! {
    static S_VAR: RefCell<S> = RefCell::new(S { i: 1, ci: 2 });
    static VS_VAR: RefCell<S> = RefCell::new(S { i: 5, ci: 6 });
}

static CS: S = S { i: 3, ci: 4 };

fn f(_p: &i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &i32) {}
fn g(_p: &i32) {}

fn main() {
    S_VAR.with(|s| {
        let s_ref = s.borrow();
        f(&s_ref.i);
        f4(&s_ref.ci);
    });

    f4(&CS.i);
    f4(&CS.ci);

    VS_VAR.with(|vs| {
        let vs_ref = vs.borrow();
        f2(&vs_ref.i);
        g(&vs_ref.ci);
    });

    S_VAR.with(|s| {
        let s_ref = s.borrow();
        if s_ref.i != 1 {
            exit(1);
        }
        if s_ref.ci != 2 {
            exit(2);
        }
    });

    if CS.i != 3 {
        exit(3);
    }
    if CS.ci != 4 {
        exit(4);
    }

    VS_VAR.with(|vs| {
        let vs_ref = vs.borrow();
        if vs_ref.i != 5 {
            exit(5);
        }
        if vs_ref.ci != 6 {
            exit(6);
        }
    });

    S_VAR.with(|s| {
        s.borrow_mut().i = 10;
    });
    S_VAR.with(|s| {
        let s_ref = s.borrow();
        if s_ref.i != 10 {
            exit(7);
        }
    });

    VS_VAR.with(|vs| {
        vs.borrow_mut().i = 20;
    });
    VS_VAR.with(|vs| {
        let vs_ref = vs.borrow();
        if vs_ref.i != 20 {
            exit(8);
        }
    });

    exit(0);
}