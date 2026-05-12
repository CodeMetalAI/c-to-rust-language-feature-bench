struct S {
    i: i32,
    ci: i32, // in Rust, mutability is determined by the binding, not by the type itself
}

static mut S: S = S { i: 1, ci: 2 };
static CS: S = S { i: 3, ci: 4 };
static mut VS: S = S { i: 5, ci: 6 };

fn f(p: &mut i32) {
    let _ = p; // simulate (void)p;
}

fn f4(p: &i32) {
    let _ = p; // simulate (void)p;
}

fn f2(p: &mut i32) {
    let _ = p; // simulate (void)p;
}

fn g(p: &i32) {
    let _ = p; // simulate (void)p;
}

fn main() {
    unsafe {
        f(&mut S.i);
        f4(&S.ci);

        f4(&CS.i);
        f4(&CS.ci);

        f2(&mut VS.i);
        g(&VS.ci);

        if S.i != 1 { return println!("1"); }
        if S.ci != 2 { return println!("2"); }
        if CS.i != 3 { return println!("3"); }
        if CS.ci != 4 { return println!("4"); }
        if VS.i != 5 { return println!("5"); }
        if VS.ci != 6 { return println!("6"); }

        // Mutate S.i and VS.i
        S.i = 10;
        if S.i != 10 { return println!("7"); }

        VS.i = 20;
        if VS.i != 20 { return println!("8"); }

        println!("0"); // equivalent to returning 0
    }
}