#[derive(Copy, Clone)]
struct S {
    pub i: i32,
    pub ci: i32,
}

static mut S_GLOBAL: S = S { i: 1, ci: 2 };
static CS_GLOBAL: S = S { i: 3, ci: 4 };
static mut VS_GLOBAL: S = S { i: 5, ci: 6 };

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() {
    use std::process;

    unsafe {
        f(&mut S_GLOBAL.i);
        f4(&S_GLOBAL.ci);

        f4(&CS_GLOBAL.i);
        f4(&CS_GLOBAL.ci);

        f2(&mut VS_GLOBAL.i);
        g(&VS_GLOBAL.ci);

        if S_GLOBAL.i != 1 {
            process::exit(1);
        }
        if S_GLOBAL.ci != 2 {
            process::exit(2);
        }
        if CS_GLOBAL.i != 3 {
            process::exit(3);
        }
        if CS_GLOBAL.ci != 4 {
            process::exit(4);
        }
        if VS_GLOBAL.i != 5 {
            process::exit(5);
        }
        if VS_GLOBAL.ci != 6 {
            process::exit(6);
        }

        S_GLOBAL.i = 10;
        if S_GLOBAL.i != 10 {
            process::exit(7);
        }

        VS_GLOBAL.i = 20;
        if VS_GLOBAL.i != 20 {
            process::exit(8);
        }
    }
}