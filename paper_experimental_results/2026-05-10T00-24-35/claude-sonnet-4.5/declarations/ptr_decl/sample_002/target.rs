use std::ptr;

type IntPtr = *mut i32;

struct Bundle {
    p_to_const: *const i32,
    const_p: *mut i32,
    const_p_via_typedef: *mut i32,
}

fn pick(p: *const i32, bias: i32) -> i32 {
    let v = unsafe { *p };
    if v & 1 != 0 {
        v + bias
    } else {
        v - bias
    }
}

fn retarget_ptr_to_const(b: &mut Bundle, a: *const i32, c: *const i32, sel: i32) {
    if sel != 0 {
        b.p_to_const = a;
    } else {
        b.p_to_const = c;
    }
}

fn same_addr(x: *const i32, y: *const i32) -> bool {
    ptr::eq(x, y)
}

fn check_bundle(b: &Bundle, expected_const_p_target: *const i32) -> i32 {
    if !same_addr(b.const_p, expected_const_p_target) {
        return 100;
    }
    if !same_addr(b.const_p_via_typedef, expected_const_p_target) {
        return 101;
    }
    0
}

fn main() {
    let mut x: i32 = 10;
    let mut y: i32 = 21;

    let ptr_to_constant: *const i32 = &x;
    let constant_ptr: *mut i32 = &mut x;
    let clarified_constant_ptr: *mut i32 = &mut x;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(&b, &x) != 0 {
        std::process::exit(1);
    }

    if unsafe { *b.p_to_const } != 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, if pick(&y, 0) > 0 { 1 } else { 0 });
    if unsafe { *b.p_to_const } != 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, if pick(&x, 1) < 0 { 1 } else { 0 });
    if unsafe { *b.p_to_const } != 21 {
        std::process::exit(4);
    }

    unsafe { *b.const_p += 5 };
    if x != 15 {
        std::process::exit(5);
    }

    unsafe { *b.const_p_via_typedef += 7 };
    if x != 22 {
        std::process::exit(6);
    }

    if check_bundle(&b, &x) != 0 {
        std::process::exit(7);
    }

    if unsafe { *b.p_to_const } != 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}