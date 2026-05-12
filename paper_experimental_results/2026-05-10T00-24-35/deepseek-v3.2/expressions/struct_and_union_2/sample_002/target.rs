struct S {
    i: std::cell::Cell<i32>,
    ci: i32,
}

static mut S_STATIC: S = S {
    i: std::cell::Cell::new(1),
    ci: 2,
};
static CS: &S = &S {
    i: std::cell::Cell::new(3),
    ci: 4,
};
static mut VS: S = S {
    i: std::cell::Cell::new(5),
    ci: 6,
};

fn f(p: &std::cell::Cell<i32>) {
    let _ = p;
}
fn f4(p: &i32) {
    let _ = p;
}
fn f2(p: &std::cell::Cell<i32>) {
    let _ = p;
}
fn g(p: &i32) {
    let _ = p;
}

fn main() {
    unsafe {
        f(&S_STATIC.i);
        f4(&S_STATIC.ci);

        f4(&CS.i.get());
        f4(&CS.ci);

        f2(&VS.i);
        g(&VS.ci);

        if S_STATIC.i.get() != 1 {
            std::process::exit(1);
        }
        if S_STATIC.ci != 2 {
            std::process::exit(2);
        }
        if CS.i.get() != 3 {
            std::process::exit(3);
        }
        if CS.ci != 4 {
            std::process::exit(4);
        }
        if VS.i.get() != 5 {
            std::process::exit(5);
        }
        if VS.ci != 6 {
            std::process::exit(6);
        }

        S_STATIC.i.set(10);
        if S_STATIC.i.get() != 10 {
            std::process::exit(7);
        }

        VS.i.set(20);
        if VS.i.get() != 20 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}