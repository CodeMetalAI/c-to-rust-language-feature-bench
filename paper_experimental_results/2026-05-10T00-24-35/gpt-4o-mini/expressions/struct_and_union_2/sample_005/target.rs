struct S {
    i: i32,
    ci: i32,
}

static mut S_INSTANCE: S = S { i: 1, ci: 2 };
static CS_INSTANCE: S = S { i: 3, ci: 4 };
static mut VS_INSTANCE: S = S { i: 5, ci: 6 };

fn f(_p: &i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() -> i32 {
    unsafe {
        f(&S_INSTANCE.i);
        f4(&S_INSTANCE.ci);

        f4(&CS_INSTANCE.i);
        f4(&CS_INSTANCE.ci);

        f2(&mut VS_INSTANCE.i);
        g(&VS_INSTANCE.ci);

        if S_INSTANCE.i != 1 {
            return 1;
        }
        if S_INSTANCE.ci != 2 {
            return 2;
        }
        if CS_INSTANCE.i != 3 {
            return 3;
        }
        if CS_INSTANCE.ci != 4 {
            return 4;
        }
        if VS_INSTANCE.i != 5 {
            return 5;
        }
        if VS_INSTANCE.ci != 6 {
            return 6;
        }

        S_INSTANCE.i = 10;
        if S_INSTANCE.i != 10 {
            return 7;
        }

        VS_INSTANCE.i = 20;
        if VS_INSTANCE.i != 20 {
            return 8;
        }
    }

    0
}