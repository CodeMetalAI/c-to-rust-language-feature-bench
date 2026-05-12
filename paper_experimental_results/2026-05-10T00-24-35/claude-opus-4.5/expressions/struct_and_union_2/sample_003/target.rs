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
    // We need to work around Rust's safety rules while maintaining safe Rust
    // Using a mutable static requires careful handling
    
    let mut s = S { i: 1, ci: 2 };
    let cs = S { i: 3, ci: 4 };
    
    // Using Cell for volatile simulation
    use std::cell::Cell;
    struct VolatileSLocal {
        i: Cell<i32>,
        ci: Cell<i32>,
    }
    let vs = VolatileSLocal {
        i: Cell::new(5),
        ci: Cell::new(6),
    };

    f(&mut s.i);
    f4(&s.ci);

    f4(&cs.i);
    f4(&cs.ci);

    // For volatile, we read/write through Cell
    let mut vs_i_temp = vs.i.get();
    f2(&mut vs_i_temp);
    vs.i.set(vs_i_temp);
    
    let vs_ci_temp = vs.ci.get();
    g(&vs_ci_temp);

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
    if vs.i.get() != 5 {
        std::process::exit(5);
    }
    if vs.ci.get() != 6 {
        std::process::exit(6);
    }

    s.i = 10;
    if s.i != 10 {
        std::process::exit(7);
    }

    vs.i.set(20);
    if vs.i.get() != 20 {
        std::process::exit(8);
    }

    std::process::exit(0);
}