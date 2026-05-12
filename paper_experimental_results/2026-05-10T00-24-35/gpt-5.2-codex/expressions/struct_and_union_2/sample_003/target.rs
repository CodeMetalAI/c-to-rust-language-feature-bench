use std::sync::Mutex;

struct S {
    i: i32,
    ci: i32,
}

static S_INST: Mutex<S> = Mutex::new(S { i: 1, ci: 2 });
static CS_INST: S = S { i: 3, ci: 4 };
static VS_INST: Mutex<S> = Mutex::new(S { i: 5, ci: 6 });

fn f(p: &i32) {
    let _ = p;
}
fn f4(p: &i32) {
    let _ = p;
}
fn f2(p: &i32) {
    let _ = p;
}
fn g(p: &i32) {
    let _ = p;
}

fn main() {
    {
        let s = S_INST.lock().unwrap();
        f(&s.i);
        f4(&s.ci);
    }

    f4(&CS_INST.i);
    f4(&CS_INST.ci);

    {
        let vs = VS_INST.lock().unwrap();
        f2(&vs.i);
        g(&vs.ci);
    }

    {
        let s = S_INST.lock().unwrap();
        if s.i != 1 {
            std::process::exit(1);
        }
        if s.ci != 2 {
            std::process::exit(2);
        }
    }

    if CS_INST.i != 3 {
        std::process::exit(3);
    }
    if CS_INST.ci != 4 {
        std::process::exit(4);
    }

    {
        let vs = VS_INST.lock().unwrap();
        if vs.i != 5 {
            std::process::exit(5);
        }
        if vs.ci != 6 {
            std::process::exit(6);
        }
    }

    {
        let mut s = S_INST.lock().unwrap();
        s.i = 10;
        if s.i != 10 {
            std::process::exit(7);
        }
    }

    {
        let mut vs = VS_INST.lock().unwrap();
        vs.i = 20;
        if vs.i != 20 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}