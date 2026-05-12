// Define a type alias for a pointer to an integer.
type IntPtr = *const i32;

// Define a struct with various pointer fields.
#[derive(Debug)]
struct Bundle {
    ptr_to_const: *const i32,
    const_ptr: *mut i32,
    const_ptr_via_typedef: IntPtr,
}

// Implement the pick function.
fn pick(ptr: *const i32, bias: i32) -> i32 {
    let v = unsafe { *ptr };
    if v & 1!= 0 {
        v + bias
    } else {
        v - bias
    }
}

// Implement the retarget_ptr_to_const function.
fn retarget_ptr_to_const(bundle: &mut Bundle, a: *const i32, c: *const i32, sel: bool) {
    if sel {
        bundle.ptr_to_const = a;
    } else {
        bundle.ptr_to_const = c;
    }
}

// Implement the same_addr function.
fn same_addr(x: *const (), y: *const ()) -> bool {
    x as usize == y as usize
}

// Implement the check_bundle function.
fn check_bundle(bundle: Bundle, expected_const_p_target: *mut i32) -> i32 {
    if!same_addr(bundle.const_ptr as *const (), expected_const_p_target as *const ()) {
        100
    } else if!same_addr(bundle.const_ptr_via_typedef as *const (), expected_const_p_target as *const ()) {
        101
    } else {
        0
    }
}

fn main() {
    let x = 10;
    let y = 21;

    let ptr_to_constant = &x as *const i32;
    let constant_ptr = &mut x as *mut i32;
    let clarified_constant_ptr = &x as *const i32;

    let mut b = Bundle {
        ptr_to_const: ptr_to_constant,
        const_ptr: constant_ptr,
        const_ptr_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(b.clone(), &mut x as *mut i32)!= 0 {
        std::process::exit(1);
    }

    let ptr_to_const_val = unsafe { *b.ptr_to_const };
    if ptr_to_const_val!= 10 {
        std::process::exit(2);
    }

    retarget_ptr_to_const(&mut b, &y as *const i32, &x as *const i32, pick(&y as *const i32, 0) > 0);
    let ptr_to_const_val = unsafe { *b.ptr_to_const };
    if ptr_to_const_val!= 21 {
        std::process::exit(3);
    }

    retarget_ptr_to_const(&mut b, &x as *const i32, &y as *const i32, pick(&x as *const i32, 1) < 0);
    let ptr_to_const_val = unsafe { *b.ptr_to_const };
    if ptr_to_const_val!= 21 {
        std::process::exit(4);
    }

    unsafe { *b.const_ptr += 5 };
    if x!= 15 {
        std::process::exit(5);
    }

    unsafe { *b.const_ptr_via_typedef += 7 };
    if x!= 22 {
        std::process::exit(6);
    }

    if check_bundle(b.clone(), &mut x as *mut i32)!= 0 {
        std::process::exit(7);
    }

    let ptr_to_const_val = unsafe { *b.ptr_to_const };
    if ptr_to_const_val!= 21 {
        std::process::exit(8);
    }

    std::process::exit(0);
}