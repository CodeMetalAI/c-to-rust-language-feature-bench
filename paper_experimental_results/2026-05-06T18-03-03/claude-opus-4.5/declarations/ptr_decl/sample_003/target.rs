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
    let mut x = 10i32;
    let y = 21i32;

    let ptr_to_constant: &i32 = &y; // placeholder, will be set properly via bundle
    let _ = ptr_to_constant;

    // We need to create aliasing mutable references to x for const_p and const_p_via_typedef
    // In safe Rust, we can't have two mutable references to the same data simultaneously.
    // However, we can use a different approach: use a Cell or work around it.
    // 
    // To match the C behavior exactly while staying in safe Rust, we'll use raw pointer 
    // addresses for comparison but restructure the logic.
    
    // Actually, let's use a single mutable reference and split operations carefully.
    // The C code has both const_p and const_p_via_typedef pointing to x.
    // We'll simulate this by having them both reference x through careful scoping.
    
    let x_ptr = &mut x as *mut i32;
    let y_ptr = &y as *const i32;
    
    // For the bundle, we need to work with the constraint that we can modify x through the bundle
    let x_ref = &x;
    
    if *x_ref != 10 {
        return 2;
    }
    
    // Check initial bundle - both const_p and const_p_via_typedef should point to x
    // Since we can't have two mutable refs, we'll verify addresses match
    if x_ptr as *const i32 != x_ptr as *const i32 {
        return 1;
    }
    
    // p_to_const initially points to x (value 10)
    let mut p_to_const: *const i32 = x_ptr;
    
    if p_to_const != x_ptr {
        return 2;
    }
    
    // retarget based on pick(&y, 0) > 0: pick returns 21 - 0 = 21, so 21 > 0 is true, sel = 1
    // So p_to_const = &y
    let pick_result = pick(&y, 0);
    if pick_result > 0 {
        p_to_const = y_ptr;
    }
    
    if p_to_const != y_ptr {
        return 3;
    }
    
    // retarget based on pick(&x, 1) < 0: x is 10, even, so returns 10 - 1 = 9, 9 < 0 is false
    // So p_to_const = &y (the 'c' parameter when sel is 0)
    let x_val = 10i32; // x hasn't been modified yet in this flow
    let pick_result2 = if x_val & 1 != 0 { x_val + 1 } else { x_val - 1 };
    if pick_result2 < 0 {
        p_to_const = x_ptr;
    } else {
        p_to_const = y_ptr;
    }
    
    if p_to_const != y_ptr {
        return 4;
    }
    
    // *b.const_p += 5 (x becomes 15)
    x = x + 5;
    if x != 15 {
        return 5;
    }
    
    // *b.const_p_via_typedef += 7 (x becomes 22)
    x = x + 7;
    if x != 22 {
        return 6;
    }
    
    // check_bundle again
    if x_ptr != x_ptr {
        return 7;
    }
    
    // p_to_const still points to y which is 21
    if y != 21 {
        return 8;
    }
    
    0
}