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
    let mut x = 10i32;
    let y = 21i32;

    let x_ptr: *const i32 = &x;

    // We need to split x into two mutable references that point to the same location.
    // In safe Rust, we can't have two mutable references to the same data.
    // However, we can use a Cell to achieve interior mutability.
    
    use std::cell::Cell;
    
    let x = Cell::new(10i32);
    let y = Cell::new(21i32);

    struct BundleCell<'a> {
        p_to_const: &'a Cell<i32>,
        const_p: &'a Cell<i32>,
        const_p_via_typedef: &'a Cell<i32>,
    }

    fn pick_cell(p: &Cell<i32>, bias: i32) -> i32 {
        let v = p.get();
        if v & 1 != 0 {
            v + bias
        } else {
            v - bias
        }
    }

    fn retarget_ptr_to_const_cell<'a>(b: &mut BundleCell<'a>, a: &'a Cell<i32>, c: &'a Cell<i32>, sel: i32) {
        if sel != 0 {
            b.p_to_const = a;
        } else {
            b.p_to_const = c;
        }
    }

    fn same_addr_cell<T, U>(x: *const T, y: *const U) -> bool {
        x as *const () == y as *const ()
    }

    fn check_bundle_cell(b: &BundleCell, expected_const_p_target: *const Cell<i32>) -> i32 {
        if !same_addr_cell(b.const_p as *const Cell<i32>, expected_const_p_target) {
            return 100;
        }
        if !same_addr_cell(b.const_p_via_typedef as *const Cell<i32>, expected_const_p_target) {
            return 101;
        }
        0
    }

    let mut b = BundleCell {
        p_to_const: &x,
        const_p: &x,
        const_p_via_typedef: &x,
    };

    if check_bundle_cell(&b, &x as *const Cell<i32>) != 0 {
        std::process::exit(1);
    }

    if b.p_to_const.get() != 10 {
        std::process::exit(2);
    }

    let sel = if pick_cell(&y, 0) > 0 { 1 } else { 0 };
    retarget_ptr_to_const_cell(&mut b, &y, &x, sel);
    if b.p_to_const.get() != 21 {
        std::process::exit(3);
    }

    let sel = if pick_cell(&x, 1) < 0 { 1 } else { 0 };
    retarget_ptr_to_const_cell(&mut b, &x, &y, sel);
    if b.p_to_const.get() != 21 {
        std::process::exit(4);
    }

    b.const_p.set(b.const_p.get() + 5);
    if x.get() != 15 {
        std::process::exit(5);
    }

    b.const_p_via_typedef.set(b.const_p_via_typedef.get() + 7);
    if x.get() != 22 {
        std::process::exit(6);
    }

    if check_bundle_cell(&b, &x as *const Cell<i32>) != 0 {
        std::process::exit(7);
    }

    if b.p_to_const.get() != 21 {
        std::process::exit(8);
    }
}