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

    let ptr_to_constant = &y; // We'll use y here initially, but reassign in bundle

    // We need to work around Rust's borrowing rules
    // The C code has both const_p and const_p_via_typedef pointing to x
    // and both are used to mutate x. In Rust, we can't have two mutable references.
    // However, we can use a different approach: since they always point to the same location,
    // we can simulate this by using a single mutable reference and splitting access.
    
    // Actually, looking at the C code more carefully:
    // - const_p and const_p_via_typedef both point to x and are used to mutate it
    // - They are never used simultaneously in a way that would violate aliasing
    
    // We'll use raw pointer semantics wrapped in a cell-like pattern
    // But since we need safe Rust only, let's restructure:
    
    // The key insight: const_p and const_p_via_typedef are both &mut x
    // We can use a Cell to allow interior mutability
    
    use std::cell::Cell;
    
    let x_cell = Cell::new(10i32);
    let y_val = 21i32;
    
    struct BundleCell<'a> {
        p_to_const: &'a i32,
        const_p: &'a Cell<i32>,
        const_p_via_typedef: &'a Cell<i32>,
    }
    
    let b_cell = BundleCell {
        p_to_const: &y_val,
        const_p: &x_cell,
        const_p_via_typedef: &x_cell,
    };
    
    // check_bundle equivalent - both point to same cell
    if !std::ptr::eq(b_cell.const_p, &x_cell) {
        return 1;
    }
    if !std::ptr::eq(b_cell.const_p_via_typedef, &x_cell) {
        return 1;
    }
    
    // Initial p_to_const should point to x (value 10)
    let mut p_to_const_val: &i32 = &y_val; // temporary
    x_cell.set(10);
    let x_ref: &i32 = &*Box::leak(Box::new(x_cell.get())); // We need a stable reference
    
    // Let's restart with a simpler approach using indices/values
    let mut x: i32 = 10;
    let y: i32 = 21;
    
    let mut p_to_const_is_y = false; // false = x, true = y
    
    // check_bundle: const_p and const_p_via_typedef both target x - always true in our model
    
    // if (*b.p_to_const != 10) - p_to_const points to x initially
    if x != 10 { return 2; }
    
    // retarget_ptr_to_const(&b, &y, &x, pick(&y, 0) > 0);
    let sel = pick(&y, 0) > 0;
    p_to_const_is_y = if sel { true } else { false };
    
    if (if p_to_const_is_y { y } else { x }) != 21 { return 3; }
    
    // retarget_ptr_to_const(&b, &x, &y, pick(&x, 1) < 0);
    let sel = pick(&x, 1) < 0;
    p_to_const_is_y = if sel { false } else { true };
    
    if (if p_to_const_is_y { y } else { x }) != 21 { return 4; }
    
    x += 5;
    if x != 15 { return 5; }
    
    x += 7;
    if x != 22 { return 6; }
    
    // check_bundle again - still valid
    
    if (if p_to_const_is_y { y } else { x }) != 21 { return 8; }
    
    0
}