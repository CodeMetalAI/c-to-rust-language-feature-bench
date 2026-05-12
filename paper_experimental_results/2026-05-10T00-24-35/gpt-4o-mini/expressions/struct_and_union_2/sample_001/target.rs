struct S {
    i: i32,
    ci: i32,
}

static mut S_STATIC: S = S { i: 1, ci: 2 };
static CS: S = S { i: 3, ci: 4 };
static mut VS: S = S { i: 5, ci: 6 };

fn f(_p: &i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() -> i32 {
    unsafe {
        f(&S_STATIC.i);
        f4(&S_STATIC.ci);

        f4(&CS.i);
        f4(&CS.ci);

        f2(&mut VS.i);
        g(&VS.ci);

        if S_STATIC.i != 1 {
            return 1;
        }
        if S_STATIC.ci != 2 {
            return 2;
        }
        if CS.i != 3 {
            return 3;
        }
        if CS.ci != 4 {
            return 4;
        }
        if VS.i != 5 {
            return 5;
        }
        if VS.ci != 6 {
            return 6;
        }

        S_STATIC.i = 10;
        if S_STATIC.i != 10 {
            return 7;
        }

        VS.i = 20;
        if VS.i != 20 {
            return 8;
        }
    }
    0
}