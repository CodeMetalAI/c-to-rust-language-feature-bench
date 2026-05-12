struct S {
    i: i32,
    ci: i32,
}

static mut S_VAR: S = S { i: 1, ci: 2 };
static CS: S = S { i: 3, ci: 4 };

use std::cell::UnsafeCell;

struct VolatileS {
    i: UnsafeCell<i32>,
    ci: UnsafeCell<i32>,
}

// Safety: We're simulating C's volatile static behavior
unsafe impl Sync for VolatileS {}

static VS: VolatileS = VolatileS {
    i: UnsafeCell::new(5),
    ci: UnsafeCell::new(6),
};

fn f(_p: &mut i32) {}
fn f4(_p: &i32) {}
fn f2(_p: &mut i32) {}
fn g(_p: &i32) {}

fn main() {
    // We need to simulate the C behavior without unsafe
    // Using a mutable local copy for s since we modify it
    let mut s = S { i: 1, ci: 2 };
    let cs = S { i: 3, ci: 4 };
    
    // Simulating volatile with explicit read/write through a cell
    use std::cell::Cell;
    let vs_i = Cell::new(5i32);
    let vs_ci = Cell::new(6i32);

    f(&mut s.i);
    f4(&s.ci);

    f4(&cs.i);
    f4(&cs.ci);

    // For volatile, we need mutable access simulation
    let mut vs_i_val = vs_i.get();
    f2(&mut vs_i_val);
    vs_i.set(vs_i_val);
    
    let vs_ci_val = vs_ci.get();
    g(&vs_ci_val);

    if s.i != 1 {
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
    if vs_i.get() != 5 {
        std::process::exit(5);
    }
    if vs_ci.get() != 6 {
        std::process::exit(6);
    }

    s.i = 10;
    if s.i != 10 {
        std::process::exit(7);
    }

    vs_i.set(20);
    if vs_i.get() != 20 {
        std::process::exit(8);
    }

    std::process::exit(0);
}