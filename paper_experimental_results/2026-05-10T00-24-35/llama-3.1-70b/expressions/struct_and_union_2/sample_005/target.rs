struct S {
    i: i32,
    ci: i32,
}

static mut s: S = S { i: 1, ci: 2 };
static cs: S = S { i: 3, ci: 4 };
static mut vs: S = S { i: 5, ci: 6 };

fn f(p: *mut i32) { unsafe { let _ = *p; } }
fn f4(p: *const i32) { let _ = p; }
fn f2(p: *mut i32) { let _ = p; }
fn g(p: *const i32) { let _ = p; }

fn main() {
    unsafe {
        f(&mut s.i);
        f4(&s.ci);

        f4(&cs.i);
        f4(&cs.ci);

        f2(&mut vs.i);
        g(&vs.ci);

        if s.i != 1 {
            std::process::exit(1);
        }
        if s.ci != 2 {
            std::process::exit(2);
        }
        if cs.i != 3 {
            std::process::exit(3);
        }
        if cs.ci != 4 {
            std::process::exit(4);
        }
        if vs.i != 5 {
            std::process::exit(5);
        }
        if vs.ci != 6 {
            std::process::exit(6);
        }

        s.i = 10;
        if s.i != 10 {
            std::process::exit(7);
        }

        vs.i = 20;
        if vs.i != 20 {
            std::process::exit(8);
        }
    }
}