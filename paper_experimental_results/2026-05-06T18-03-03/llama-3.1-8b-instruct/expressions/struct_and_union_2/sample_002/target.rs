struct S {
    i: i32,
    ci: i32,
}

static mut S: S = S { i: 1, ci: 2 };
static const CS: S = S { i: 3, ci: 4 };
static VS: S = S { i: 5, ci: 6 };

fn f(_p: &i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &i32) {}
fn g(_p: &i32) {}

fn main() {
    f(&s.i);
    f4(&s.ci);

    f4(&CS.i);
    f4(&CS.ci);

    f2(&VS.i);
    g(&VS.ci);

    if s.i != 1 {
        return 1;
    }
    if s.ci != 2 {
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

    s.i = 10;
    if s.i != 10 {
        return 7;
    }

    VS.i = 20;
    if VS.i != 20 {
        return 8;
    }

    0
}