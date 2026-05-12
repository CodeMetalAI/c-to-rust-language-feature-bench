struct S {
    i: i32,
    ci: i32,
}

static S1: S = S { i: 1, ci: 2 };
static CS: S = S { i: 3, ci: 4 };
static VS: std::cell::Cell<S> = std::cell::Cell::new(S { i: 5, ci: 6 });

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &std::cell::Cell<i32>) {}
fn g(_p: &std::cell::Cell<i32>) {}

fn main() {
    f(&mut S1.i);
    f4(&S1.ci);

    f4(&CS.i);
    f4(&CS.ci);

    f2(&VS.take().i);
    g(&VS.take().ci);

    if S1.i!= 1 {
        std::process::exit(1);
    }
    if S1.ci!= 2 {
        std::process::exit(2);
    }
    if CS.i!= 3 {
        std::process::exit(3);
    }
    if CS.ci!= 4 {
        std::process::exit(4);
    }
    if VS.take().i!= 5 {
        std::process::exit(5);
    }
    if VS.take().ci!= 6 {
        std::process::exit(6);
    }

    S1.i = 10;
    if S1.i!= 10 {
        std::process::exit(7);
    }

    VS.take_mut().i = 20;
    if VS.take().i!= 20 {
        std::process::exit(8);
    }

    std::process::exit(0);
}