#[derive(Clone, Copy)]
struct S {
    i: f64,
}

static mut G_U1_F2_I: f64 = 0.0;
static mut G_U2_F3_I: f64 = 0.0;

fn f() -> S {
    unsafe { S { i: G_U1_F2_I } }
}

fn foo() -> i32 {
    1
}

fn main() {
    unsafe {
        G_U1_F2_I = 1.0;
        G_U2_F3_I = 1.0;
        G_U1_F2_I = f().i;
        G_U2_F3_I = f().i;
        let val = foo() as f64;
        let sum = G_U1_F2_I + G_U2_F3_I + val;
        std::process::exit(if sum == 3.0 { 0 } else { 1 });
    }
}