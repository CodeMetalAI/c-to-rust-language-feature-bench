use std::cell::Cell;
use std::sync::atomic::{AtomicI32, Ordering};

struct S {
    i: Cell<i32>,
    ci: i32,
}

static S_S: S = S {
    i: Cell::new(1),
    ci: 2,
};

static CS_S: &S = &S {
    i: Cell::new(3),
    ci: 4,
};

static VS_I: AtomicI32 = AtomicI32::new(5);
static VS_CI: AtomicI32 = AtomicI32::new(6);

fn f(p: &Cell<i32>) {
    let _ = p;
}

fn f4(p: &i32) {
    let _ = p;
}

fn f2(p: &AtomicI32) {
    let _ = p;
}

fn g(p: &AtomicI32) {
    let _ = p;
}

fn main() {
    f(&S_S.i);
    f4(&S_S.ci);

    f4(&CS_S.i.get());
    f4(&CS_S.ci);

    f2(&VS_I);
    g(&VS_CI);

    if S_S.i.get() != 1 {
        std::process::exit(1);
    }
    if S_S.ci != 2 {
        std::process::exit(2);
    }
    if CS_S.i.get() != 3 {
        std::process::exit(3);
    }
    if CS_S.ci != 4 {
        std::process::exit(4);
    }
    if VS_I.load(Ordering::SeqCst) != 5 {
        std::process::exit(5);
    }
    if VS_CI.load(Ordering::SeqCst) != 6 {
        std::process::exit(6);
    }

    S_S.i.set(10);
    if S_S.i.get() != 10 {
        std::process::exit(7);
    }

    VS_I.store(20, Ordering::SeqCst);
    if VS_I.load(Ordering::SeqCst) != 20 {
        std::process::exit(8);
    }

    std::process::exit(0);
}