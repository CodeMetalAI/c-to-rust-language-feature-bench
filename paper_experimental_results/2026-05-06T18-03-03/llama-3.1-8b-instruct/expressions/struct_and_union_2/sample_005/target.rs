struct S {
    i: i32,
    ci: i32,
}

static mut S: S = S { i: 1, ci: 2 };
static const CS: S = S { i: 3, ci: 4 };
static volatile VS: S = S { i: 5, ci: 6 };

fn f(p: &i32) {}
fn f4(p: &i32) {}
fn f2(p: &i32) {}
fn g(p: &i32) {}

fn main() {
    f(&s.i);
    f4(&s.ci);

    f4(&cs.i);
    f4(&cs.ci);

    f2(&vs.i);
    g(&vs.ci);

    if s.i != 1 {
        return 1;
    }
    if s.ci != 2 {
        return 2;
    }
    if cs.i != 3 {
        return 3;
    }
    if cs.ci != 4 {
        return 4;
    }
    if vs.i != 5 {
        return 5;
    }
    if vs.ci != 6 {
        return 6;
    }

    s.i = 10;
    if s.i != 10 {
        return 7;
    }

    vs.i = 20;
    if vs.i != 20 {
        return 8;
    }

    return 0;
}