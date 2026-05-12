use std::process::exit;
use std::sync::Mutex;

#[derive(Copy, Clone)]
struct S {
    i: i32,
    ci: i32,
}

static S_STATIC: Mutex<S> = Mutex::new(S { i: 1, ci: 2 });
static CS: S = S { i: 3, ci: 4 };
static VS: Mutex<S> = Mutex::new(S { i: 5, ci: 6 });

fn f(p: &mut i32) {
    let _ = p;
}
fn f4(p: &i32) {
    let _ = p;
}
fn f2(p: &mut i32) {
    let _ = p;
}
fn g(p: &i32) {
    let _ = p;
}

fn main() {
    {
        let mut guard = S_STATIC.lock().unwrap();
        f(&mut guard.i);
        f4(&guard.ci);
    }

    f4(&CS.i);
    f4(&CS.ci);

    {
        let mut guard = VS.lock().unwrap();
        f2(&mut guard.i);
        g(&guard.ci);
    }

    {
        let guard = S_STATIC.lock().unwrap();
        if guard.i != 1 {
            exit(1);
        }
        if guard.ci != 2 {
            exit(2);
        }
    }
    if CS.i != 3 {
        exit(3);
    }
    if CS.ci != 4 {
        exit(4);
    }
    {
        let guard = VS.lock().unwrap();
        if guard.i != 5 {
            exit(5);
        }
        if guard.ci != 6 {
            exit(6);
        }
    }

    {
        let mut guard = S_STATIC.lock().unwrap();
        guard.i = 10;
        if guard.i != 10 {
            exit(7);
        }
    }

    {
        let mut guard = VS.lock().unwrap();
        guard.i = 20;
        if guard.i != 20 {
            exit(8);
        }
    }

    exit(0);
}