fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() {
    let mut s = (1, 2);
    let cs = (3, 4);
    let mut vs = (5, 6);

    f(&mut s.0);
    f4(&s.1);

    f4(&cs.0);
    f4(&cs.1);

    f2(&mut vs.0);
    g(&vs.1);

    if s.0 != 1 {
        return 1;
    }
    if s.1 != 2 {
        return 2;
    }
    if cs.0 != 3 {
        return 3;
    }
    if cs.1 != 4 {
        return 4;
    }
    if vs.0 != 5 {
        return 5;
    }
    if vs.1 != 6 {
        return 6;
    }

    s.0 = 10;
    if s.0 != 10 {
        return 7;
    }

    vs.0 = 20;
    if vs.0 != 20 {
        return 8;
    }

    std::process::exit(0);
}