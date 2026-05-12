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
    // We need to use a different approach since we can't use unsafe
    // Simulate the behavior using interior mutability with Cell
    use std::cell::Cell;
    
    let s = Cell::new(S { i: 1, ci: 2 });
    let cs = S { i: 3, ci: 4 };
    let vs = Cell::new(S { i: 5, ci: 6 });
    
    // Simulating the function calls - in safe Rust we work with values
    let mut s_val = s.get();
    f(&mut s_val.i);
    s.set(s_val);
    
    let s_val = s.get();
    f4(&s_val.ci);
    
    f4(&cs.i);
    f4(&cs.ci);
    
    let mut vs_val = vs.get();
    f2(&mut vs_val.i);
    vs.set(vs_val);
    
    let vs_val = vs.get();
    g(&vs_val.ci);
    
    let s_val = s.get();
    if s_val.i != 1 {
        std::process::exit(1);
    }
    if s_val.ci != 2 {
        std::process::exit(2);
    }
    if cs.i != 3 {
        std::process::exit(3);
    }
    if cs.ci != 4 {
        std::process::exit(4);
    }
    let vs_val = vs.get();
    if vs_val.i != 5 {
        std::process::exit(5);
    }
    if vs_val.ci != 6 {
        std::process::exit(6);
    }
    
    let mut s_val = s.get();
    s_val.i = 10;
    s.set(s_val);
    let s_val = s.get();
    if s_val.i != 10 {
        std::process::exit(7);
    }
    
    let mut vs_val = vs.get();
    vs_val.i = 20;
    vs.set(vs_val);
    let vs_val = vs.get();
    if vs_val.i != 20 {
        std::process::exit(8);
    }
    
    std::process::exit(0);
}