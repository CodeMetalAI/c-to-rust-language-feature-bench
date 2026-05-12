#[derive(Clone, Copy)]
struct S {
    i: f64,
}

static mut G: f64 = 0.0;

fn f() -> S {
    S { i: unsafe { G } }
}

fn foo() -> i32 {
    1
}

fn main() {
    unsafe {
        G = 1.0;
        G = 1.0;
        G = f().i;
        G = f().i;
    }
    let val = foo() as f64;
    let result = unsafe { G + G + val == 3.0 };
    std::process::exit(if result { 0 } else { 1 });
}