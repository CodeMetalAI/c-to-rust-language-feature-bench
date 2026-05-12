struct S {
    i: i32,
    ci: i32,
}

static mut S_VAR: S = S { i: 1, ci: 2 };
static CS: S = S { i: 3, ci: 4 };

use std::cell::UnsafeCell;

struct VolatileS {
    inner: UnsafeCell<S>,
}

// Safety: We're simulating C's volatile static behavior
unsafe impl Sync for VolatileS {}

static VS: VolatileS = VolatileS {
    inner: UnsafeCell::new(S { i: 5, ci: 6 }),
};

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() {
    // We need to simulate the C behavior without unsafe
    // Using a different approach with interior mutability
    
    use std::cell::Cell;
    
    struct SafeS {
        i: Cell<i32>,
        ci: i32,
    }
    
    let s = SafeS { i: Cell::new(1), ci: 2 };
    let cs = S { i: 3, ci: 4 };
    let vs = SafeS { i: Cell::new(5), ci: 6 };
    
    // Simulating the function calls (they don't do anything)
    let mut temp_i = s.i.get();
    f(&mut temp_i);
    f4(&s.ci);
    
    f4(&cs.i);
    f4(&cs.ci);
    
    let mut temp_vs_i = vs.i.get();
    f2(&mut temp_vs_i);
    g(&vs.ci);
    
    if s.i.get() != 1 {
        std::process::exit(1);
    }
    if s.ci != 2 {
        std::process::exit(2);
    }
    if cs.i != 3 {
        std::process::exit(3);
    }
    if cs.ci != 4 {
        std::process::exit(4);
    }
    if vs.i.get() != 5 {
        std::process::exit(5);
    }
    if vs.ci != 6 {
        std::process::exit(6);
    }
    
    s.i.set(10);
    if s.i.get() != 10 {
        std::process::exit(7);
    }
    
    vs.i.set(20);
    if vs.i.get() != 20 {
        std::process::exit(8);
    }
}