struct S {
    i: i32,
    ci: i32,
}

static mut S_INSTANCE: S = S { i: 1, ci: 2 };
static CS_INSTANCE: S = S { i: 3, ci: 4 };
static mut VS_INSTANCE: S = S { i: 5, ci: 6 };

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &std::cell::Cell<i32>) {}
fn g(_p: &std::cell::Cell<i32>) {}

fn main() {
    unsafe {
        f(&mut S_INSTANCE.i);
        f4(&S_INSTANCE.ci);

        f4(&CS_INSTANCE.i);
        f4(&CS_INSTANCE.ci);

        f2(&std::cell::Cell::new(VS_INSTANCE.i));
        g(&std::cell::Cell::new(VS_INSTANCE.ci));

        if S_INSTANCE.i != 1 {
            std::process::exit(1);
        }
        if S_INSTANCE.ci != 2 {
            std::process::exit(2);
        }
        if CS_INSTANCE.i != 3 {
            std::process::exit(3);
        }
        if CS_INSTANCE.ci != 4 {
            std::process::exit(4);
        }
        if VS_INSTANCE.i != 5 {
            std::process::exit(5);
        }
        if VS_INSTANCE.ci != 6 {
            std::process::exit(6);
        }

        S_INSTANCE.i = 10;
        if S_INSTANCE.i != 10 {
            std::process::exit(7);
        }

        VS_INSTANCE.i = 20;
        if VS_INSTANCE.i != 20 {
            std::process::exit(8);
        }
    }
}