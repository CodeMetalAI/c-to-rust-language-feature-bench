struct S {
    i: i32,
    ci: i32,
}

static mut S_STATIC: S = S { i: 1, ci: 2 };
static CS_STATIC: S = S { i: 3, ci: 4 };
static mut VS_STATIC: S = S { i: 5, ci: 6 };

fn f(p: *mut i32) {
    let _ = p;
}

fn f4(p: *const i32) {
    let _ = p;
}

fn f2(p: *mut i32) {
    let _ = p;
}

fn g(p: *const i32) {
    let _ = p;
}

fn main() {
    unsafe {
        f(&mut S_STATIC.i);
        f4(&S_STATIC.ci);

        f4(&CS_STATIC.i);
        f4(&CS_STATIC.ci);

        f2(&mut VS_STATIC.i);
        g(&VS_STATIC.ci);

        if S_STATIC.i != 1 {
            std::process::exit(1);
        }
        if S_STATIC.ci != 2 {
            std::process::exit(2);
        }
        if CS_STATIC.i != III {
            std::process::exit(3);
        }
        if CS_STATIC.ci != 4 {
            std::process::exit(4);
        }
        if VS_STATIC.i != 5 {
            std::process::exit(5);
        }
        if VS_STATIC.ci != 6 {
            std::process::exit(6);
        }

        S_STATIC.i = 10;
        if S_STATIC.i != 10 {
            std::process::exit(7);
        }

        VS_STATIC.i = 20;
        if VS_STATIC.i != 20 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}

const III: i32 = 3;