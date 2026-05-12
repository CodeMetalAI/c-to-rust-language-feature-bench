// bundle.rs
// Define a type alias for a pointer to an integer.
type IntPtr = *const i32;

// Define a struct to hold the pointers.
struct Bundle {
    p_to_const: *const i32,
    const_p: *mut i32,
    const_p_via_typedef: IntPtr,
}

// Define a function to pick a value based on a condition.
fn pick(p: &i32, bias: i32) -> i32 {
    let v = *p;
    if v & 1 {
        v + bias
    } else {
        v - bias
    }
}

// Define a function to retarget a pointer to const.
fn retarget_ptr_to_const(b: &mut Bundle, a: &i32, c: &i32, sel: bool) {
    if sel {
        b.p_to_const = a as *const i32;
    } else {
        b.p_to_const = c as *const i32;
    }
}

// Define a function to check if two pointers are the same.
fn same_addr(x: *const (), y: *const ()) -> bool {
    x == y
}

// Define a function to check the bundle.
fn check_bundle(b: Bundle, expected_const_p_target: &i32) -> i32 {
    if!same_addr(b.const_p, expected_const_p_target) {
        100
    } else if!same_addr(b.const_p_via_typedef, expected_const_p_target) {
        101
    } else {
        0
    }
}

fn main() {
    let mut x = 10;
    let y = 21;

    let ptr_to_constant = &x;

    let constant_ptr = &mut x;
    let clarified_constant_ptr: IntPtr = &x;

    let mut b = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(b, &x)!= 0 {
        std::process::exit(1);
    }

    if *b.p_to_const!= 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y, &x, pick(&y, 0) > 0);
    if *b.p_to_const!= 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x, &y, pick(&x, 1) < 0);
    if *b.p_to_const!= 21 {
        std::process::exit(4);
    }

    *b.const_p += 5;
    if x!= 15 {
        std::process::exit(5);
    }

    *b.const_p_via_typedef += 7;
    if x!= 22 {
        std::process::exit(6);
    }

    if check_bundle(b, &x)!= 0 {
        std::process::exit(7);
    }

    if *b.p_to_const!= 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}