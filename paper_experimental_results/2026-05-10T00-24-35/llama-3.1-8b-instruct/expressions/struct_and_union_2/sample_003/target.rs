struct S {
    i: i32,
    ci: i32,
}

static mut S: S = S { i: 1, ci: 2 };
static const CS: S = S { i: 3, ci: 4 };
static mut VS: S = S { i: 5, ci: 6 };

fn f(_p: &i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &i32) {}
fn g(_p: &i32) {}

fn main() {
    f(&mut S.i);
    f4(&CS.ci);

    f4(&CS.i);
    f4(&CS.ci);

    f2(&mut VS.i);
    g(&VS.ci);

    if S.i != 1 {
        panic!("s.i != 1");
    }
    if S.ci != 2 {
        panic!("s.ci != 2");
    }
    if CS.i != 3 {
        panic!("cs.i != 3");
    }
    if CS.ci != 4 {
        panic!("cs.ci != 4");
    }
    if VS.i != 5 {
        panic!("vs.i != 5");
    }
    if VS.ci != 6 {
        panic!("vs.ci != 6");
    }

    S.i = 10;
    if S.i != 10 {
        panic!("s.i != 10");
    }

    VS.i = 20;
    if VS.i != 20 {
        panic!("vs.i != 20");
    }

    println!("0");
}