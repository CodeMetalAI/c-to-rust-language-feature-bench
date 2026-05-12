fn main() -> i32 {
    let x = 42;
    let p = &x;

    if p as *const _ != &x as *const _ {
        return 1;
    }

    let p_end1 = unsafe { (p as *const i32).add(1) }; // Simulate pointer arithmetic
    let p_end2 = unsafe { (&x as *const i32).add(1) }; // Simulate pointer arithmetic

    if p_end1 != p_end2 {
        return 2;
    }

    if p_end1 == p as *const i32 {
        return 3;
    }

    0
}