struct S {
    i: i32,
    ci: i32,
}

fn main() {
    let s = S { i: 1, ci: 2 };
    let cs = S { i: 3, ci: 4 };
    let vs = S { i: 5, ci: 6 };

    fn f(_p: &mut i32) {}
    fn f4(_p: &i32) {}
    fn f2(_p: &mut i32) {}
    fn g(_p: &i32) {}

    f(&mut s.i);
    f4(&s.ci);

    f4(&cs.i);
    f4(&cs.ci);

    f2(&mut vs.i);
    g(&vs.ci);

    if s.i != 1 {
        return;
    }
    if s.ci != 2 {
        return;
    }
    if cs.i != 3 {
        return;
    }
    if cs.ci != 4 {
        return;
    }
    if vs.i != 5 {
        return;
    }
    if vs.ci != 6 {
        return;
    }

    s.i = 10;
    if s.i != 10 {
        return;
    }

    vs.i = 20;
    if vs.i != 20 {
        return;
    }
}