struct S {
    i: i32,
    ci: i32,
}

static mut S1: S = S { i: 1, ci: 2 };
static S2: S = S { i: 3, ci: 4 };
static mut S3: S = S { i: 5, ci: 6 };

fn f(p: &mut i32) { }
fn f4(p: &i32) { }
fn f2(p: &mut i32) { }
fn g(p: &i32) { }

fn main() {
    unsafe {
        f(&mut S1.i);
        f4(&S1.ci);

        f4(&S2.i);
        f4(&S2.ci);

        f2(&mut S3.i);
        g(&S3.ci);

        if S1.i!= 1 {
            std::process::exit(1);
        }
        if S1.ci!= 2 {
            std::process::exit(2);
        }
        if S2.i!= 3 {
            std::process::exit(3);
        }
        if S2.ci!= 4 {
            std::process::exit(4);
        }
        if S3.i!= 5 {
            std::process::exit(5);
        }
        if S3.ci!= 6 {
            std::process::exit(6);
        }

        S1.i = 10;
        if S1.i!= 10 {
            std::process::exit(7);
        }

        S3.i = 20;
        if S3.i!= 20 {
            std::process::exit(8);
        }
    }
    std::process::exit(0);
}