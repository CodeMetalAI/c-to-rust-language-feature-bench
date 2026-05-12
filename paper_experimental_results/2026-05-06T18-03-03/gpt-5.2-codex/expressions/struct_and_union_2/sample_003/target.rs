use std::sync::Mutex;

#[derive(Copy, Clone)]
struct S {
    i: i32,
    ci: i32,
}

static S_MUTEX: Mutex<S> = Mutex::new(S { i: 1, ci: 2 });
static CS: S = S { i: 3, ci: 4 };
static VS_MUTEX: Mutex<S> = Mutex::new(S { i: 5, ci: 6 });

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn run() -> i32 {
    {
        let mut s = S_MUTEX.lock().unwrap();
        f(&mut s.i);
        f4(&s.ci);
    }

    f4(&CS.i);
    f4(&CS.ci);

    {
        let mut vs = VS_MUTEX.lock().unwrap();
        f2(&mut vs.i);
        g(&vs.ci);
    }

    {
        let s = S_MUTEX.lock().unwrap();
        if s.i != 1 {
            return 1;
        }
        if s.ci != 2 {
            return 2;
        }
    }
    if CS.i != 3 {
        return 3;
    }
    if CS.ci != 4 {
        return 4;
    }
    {
        let vs = VS_MUTEX.lock().unwrap();
        if vs.i != 5 {
            return 5;
        }
        if vs.ci != 6 {
            return 6;
        }
    }

    {
        let mut s = S_MUTEX.lock().unwrap();
        s.i = 10;
        if s.i != 10 {
            return 7;
        }
    }

    {
        let mut vs = VS_MUTEX.lock().unwrap();
        vs.i = 20;
        if vs.i != 20 {
            return 8;
        }
    }

    0
}

fn main() {
    std::process::exit(run());
}