// ptr_decl.rs
type IntPtr = Box<i32>;

struct Bundle<'a> {
    p_to_const: &'a i32,
    const_p: &'a i32,
    const_p_via_typedef: &'a IntPtr,
}

fn pick(p: &'a i32, bias: i32) -> i32 {
    *p + if *p & 1 { bias } else { -bias }
}

fn retarget_ptr_to_const(bundle: &mut Bundle, a: &'a i32, c: &'a i32, sel: bool) {
    if sel {
        bundle.p_to_const = a;
    } else {
        bundle.p_to_const = c;
    }
}

fn same_addr(x: &dyn Any, y: &dyn Any) -> bool {
    x as *const _ == y as *const _
}

fn check_bundle(bundle: Bundle, expected_const_p_target: &'a i32) -> i32 {
    if bundle.const_p != expected_const_p_target {
        return 100;
    }
    if bundle.const_p_via_typedef.as_ref().unwrap() != expected_const_p_target {
        return 101;
    }
    0
}

fn main() {
    let x = 10;
    let y = 21;

    let ptr_to_constant = &x;

    let constant_ptr = &x;
    let clarified_constant_ptr = Box::new(x);

    let mut bundle = Bundle {
        p_to_const: ptr_to_constant,
        const_p: constant_ptr,
        const_p_via_typedef: clarified_constant_ptr,
    };

    if check_bundle(bundle, &x) != 0 {
        panic!();
    }

    if *bundle.p_to_const != 10 {
        panic!();
    }

    retarget_ptr_to_const(&mut bundle, &y, &x, pick(&y, 0) > 0);
    if *bundle.p_to_const != 21 {
        panic!();
    }

    retarget_ptr_to_const(&mut bundle, &x, &y, pick(&x, 1) < 0);
    if *bundle.p_to_const != 21 {
        panic!();
    }

    *bundle.const_p += 5;
    if x != 15 {
        panic!();
    }

    *bundle.const_p_via_typedef += 7;
    if x != 22 {
        panic!();
    }

    if check_bundle(bundle, &x) != 0 {
        panic!();
    }

    if *bundle.p_to_const != 21 {
        panic!();
    }

    std::process::exit(0);
}