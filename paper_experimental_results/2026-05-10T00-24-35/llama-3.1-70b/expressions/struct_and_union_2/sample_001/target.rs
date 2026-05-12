struct S {
    i: i32,
    ci: i32,
}

static S: S = S { i: 1, ci: 2 };
static CS: S = S { i: 3, ci: 4 };
static VS: S = S { i: 5, ci: 6 };

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() {
    f(&mut S.i);
    f4(&S.ci);

    f4(&CS.i);
    f4(&CS.ci);

    f2(&mut VS.i);
    g(&VS.ci);

    assert_eq!(S.i, 1);
    assert_eq!(S.ci, 2);
    assert_eq!(CS.i, 3);
    assert_eq!(CS.ci, 4);
    assert_eq!(VS.i, 5);
    assert_eq!(VS.ci, 6);

    S.i = 10;
    assert_eq!(S.i, 10);

    VS.i = 20;
    assert_eq!(VS.i, 20);
}