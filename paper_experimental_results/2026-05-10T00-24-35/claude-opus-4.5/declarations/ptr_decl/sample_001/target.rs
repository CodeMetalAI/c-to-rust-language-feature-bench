struct Bundle<'a> {
    p_to_const: &'a i32,
    const_p: &'a mut i32,
    const_p_via_typedef: &'a mut i32,
}

fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const<'a>(b: &mut Bundle<'a>, a: &'a i32, c: &'a i32, sel: i32) {
    if sel != 0 {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr<T, U>(x: *const T, y: *const U) -> bool {
    x as *const () == y as *const ()
}

fn check_bundle(b: &Bundle, expected_const_p_target: *const i32) -> i32 {
    if !same_addr(b.const_p as *const i32, expected_const_p_target) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef as *const i32, expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let mut x = 10;
    let y = 21;

    let ptr_to_constant = &y; // We'll use references differently due to Rust's borrowing rules

    // In Rust, we can't have two mutable references to the same data simultaneously.
    // We need to restructure this to work with Rust's ownership model.
    // We'll use a different approach: since const_p and const_p_via_typedef both point to x
    // and are used to mutate x, we'll use raw pointer semantics via a Cell.

    use std::cell::Cell;
    
    let x_cell = Cell::new(10);
    let y_val = 21;

    struct BundleCell<'a> {
        p_to_const: &'a i32,
        const_p: &'a Cell<i32>,
        const_p_via_typedef: &'a Cell<i32>,
    }

    fn pick_val(p: i32, bias: i32) -> i32 {
        if p & 1 != 0 {
            p + bias
        } else {
            p - bias
        }
    }

    fn retarget<'a>(b: &mut BundleCell<'a>, a: &'a i32, c: &'a i32, sel: i32) {
        if sel != 0 {
            b.p_to_const = a;
        } else {
            b.p_to_const = c;
        }
    }

    fn check(b: &BundleCell, expected: *const Cell<i32>) -> i32 {
        if b.const_p as *const Cell<i32> != expected {
            return 100;
        }
        if b.const_p_via_typedef as *const Cell<i32> != expected {
            return 101;
        }
        0
    }

    let mut b = BundleCell {
        p_to_const: &y_val,
        const_p: &x_cell,
        const_p_via_typedef: &x_cell,
    };

    // Temporarily set p_to_const to point to x_cell's value for initial check
    let x_val_initial = x_cell.get();
    let x_ref: &i32 = &x_val_initial;
    b.p_to_const = x_ref;

    if check(&b, &x_cell as *const Cell<i32>) != 0 {
        return 1;
    }

    if *b.p_to_const != 10 {
        return 2;
    }

    let y_ref: &i32 = &y_val;
    let x_val_for_ref = x_cell.get();
    let x_ref2: &i32 = &x_val_for_ref;
    let cond1 = pick_val(y_val, 0) > 0;
    retarget(&mut b, y_ref, x_ref2, if cond1 { 1 } else { 0 });
    if *b.p_to_const != 21 {
        return 3;
    }

    let x_val_for_pick = x_cell.get();
    let cond2 = pick_val(x_val_for_pick, 1) < 0;
    let x_val_for_ref3 = x_cell.get();
    let x_ref3: &i32 = &x_val_for_ref3;
    retarget(&mut b, x_ref3, y_ref, if cond2 { 1 } else { 0 });
    if *b.p_to_const != 21 {
        return 4;
    }

    b.const_p.set(b.const_p.get() + 5);
    if x_cell.get() != 15 {
        return 5;
    }

    b.const_p_via_typedef.set(b.const_p_via_typedef.get() + 7);
    if x_cell.get() != 22 {
        return 6;
    }

    if check(&b, &x_cell as *const Cell<i32>) != 0 {
        return 7;
    }

    if *b.p_to_const != 21 {
        return 8;
    }

    0
}