struct S {
    i: i32,
    ci: i32,
}

static mut S_VAR: S = S { i: 1, ci: 2 };
static CS: S = S { i: 3, ci: 4 };
static mut VS: S = S { i: 5, ci: 6 };

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() {
    // Using unsafe blocks to access mutable statics, but the references themselves are safe
    let exit_code = {
        f(unsafe { &mut S_VAR.i });
        f4(unsafe { &S_VAR.ci });

        f4(&CS.i);
        f4(&CS.ci);

        f2(unsafe { &mut VS.i });
        g(unsafe { &VS.ci });

        if unsafe { S_VAR.i } != 1 {
            1
        } else if unsafe { S_VAR.ci } != 2 {
            2
        } else if CS.i != 3 {
            3
        } else if CS.ci != 4 {
            4
        } else if unsafe { VS.i } != 5 {
            5
        } else if unsafe { VS.ci } != 6 {
            6
        } else {
            unsafe { S_VAR.i = 10 };
            if unsafe { S_VAR.i } != 10 {
                7
            } else {
                unsafe { VS.i = 20 };
                if unsafe { VS.i } != 20 {
                    8
                } else {
                    0
                }
            }
        }
    };

    std::process::exit(exit_code);
}