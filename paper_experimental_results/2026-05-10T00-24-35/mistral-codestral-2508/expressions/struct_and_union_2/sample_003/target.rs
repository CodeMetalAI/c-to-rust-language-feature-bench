struct S {
    i: i32,
    ci: i32,
}

static mut S: S = S { i: 1, ci: 2 };
static mut CS: S = S { i: 3, ci: 4 };
static mut VS: S = S { i: 5, ci: 6 };

fn f(p: *mut i32) {
    unsafe { *p = *p; }
}

fn f4(p: *const i32) {
    unsafe { let _ = *p; }
}

fn f2(p: *mut i32) {
    unsafe { *p = *p; }
}

fn g(p: *const i32) {
    unsafe { let _ = *p; }
}

fn main() {
    unsafe {
        f(&mut S.i);
        f4(&S.ci);

        f4(&CS.i);
        f4(&CS.ci);

        f2(&mut VS.i);
        g(&VS.ci);

        if S.i != 1 {
            std::process::exit(1);
        }
        if S.ci != 2 {
            std::process::exit(2);
        }
        if CS.i != 3 {
            std::process::exit(3);
        }
        if CS.ci != 4 {
            std::process::exit(4);
        }
        if VS.i != 5 {
            std::process::exit(5);
        }
        if VS.ci != 6 {
            std::process::exit(6);
        }

        S.i = 10;
        if S.i != 10 {
            std::process::exit(7);
        }

        VS.i = 20;
        if VS.i != 20 {
            std::process::exit(8);
        }
    }
}