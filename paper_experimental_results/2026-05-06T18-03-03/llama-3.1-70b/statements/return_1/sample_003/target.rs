#[derive(Debug, Default, Copy, Clone)]
struct S {
    i: f64,
}

#[derive(Debug, Default, Copy, Clone)]
union U {
    u1: (i32, S),
    u2: (S, i32),
}

fn f() -> S {
    unsafe { *(&g as *const U).as_ref().unwrap().u1.1 }
}

fn foo() -> f64 {
    1.0
}

fn main() {
    unsafe {
        *(&mut g as *mut U).as_mut().unwrap().u1.1 = S { i: 1.0 };
        *(&mut g as *mut U).as_mut().unwrap().u2.0 = S { i: 1.0 };
        *(&mut g as *mut U).as_mut().unwrap().u1.1 = f();
        *(&mut g as *mut U).as_mut().unwrap().u2.0 = f();
    }
    let val = foo();
    let result = unsafe {
        *(&g as *const U).as_ref().unwrap().u1.1.i
            + *(&g as *const U).as_ref().unwrap().u2.0.i
            + val
    };
    if result == 3.0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

static mut g: U = U::default();