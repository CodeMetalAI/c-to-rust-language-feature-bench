use std::sync::Mutex;

#[derive(Clone, Copy)]
struct S {
    i: i32,
    ci: i32,
}

static S_VAR: Mutex<S> = Mutex::new(S { i: 1, ci: 2 });
static CS: S = S { i: 3, ci: 4 };
static VS: Mutex<S> = Mutex::new(S { i: 5, ci: 6 });

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() {
    {
        let mut s = S_VAR.lock().unwrap();
        f(&mut s.i);
        f4(&s.ci);
    }

    f4(&CS.i);
    f4(&CS.ci);

    {
        let mut vs = VS.lock().unwrap();
        f2(&mut vs.i);
        g(&vs.ci);
    }

    {
        let s = S_VAR.lock().unwrap();
        if s.i != 1 {
            std::process::exit(1);
        }
        if s.ci != 2 {
            std::process::exit(2);
        }
    }
    if CS.i != 3 {
        std::process::exit(3);
    }
    if CS.ci != 4 {
        std::process::exit(4);
    }
    {
        let vs = VS.lock().unwrap();
        if vs.i != 5 {
            std::process::exit(5);
        }
        if vs.ci != 6 {
            std::process::exit(6);
        }
    }

    {
        let mut s = S_VAR.lock().unwrap();
        s.i = 10;
        if s.i != 10 {
            std::process::exit(7);
        }
    }

    {
        let mut vs = VS.lock().unwrap();
        vs.i = 20;
        if vs.i != 20 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}