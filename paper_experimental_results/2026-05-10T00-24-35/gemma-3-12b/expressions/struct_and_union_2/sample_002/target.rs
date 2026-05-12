#[derive(Debug, PartialEq)]
struct S {
    i: i32,
    ci: i32,
}

static S: S = S { i: 1, ci: 2 };
static const CS: S = S { i: 3, ci: 4 };
static VOLATILE S VS: S = S { i: 5, ci: 6 };

fn f(p: &i32) {
    (p as i32);
}

fn f4(p: &i32) {
    (p as i32);
}

fn f2(p: &i32) {
    (p as i32);
}

fn g(p: &i32) {
    (p as i32);
}

fn main() {
    f(&S.i);
    f4(&S.ci);

    f4(&CS.i);
    f4(&CS.ci);

    f2(&VS.i);
    g(&VS.ci);

    if S.i != 1 {
        return 1;
    }
    if S.ci != 2 {
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

    S.i = 10;
    if S.i != 10 {
        return 7;
    }

    VS.i = 20;
    if VS.i != 20 {
        return 8;
    }

    0
}