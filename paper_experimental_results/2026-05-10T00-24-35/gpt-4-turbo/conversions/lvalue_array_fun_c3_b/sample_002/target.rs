fn main() {
    let mut a = [10, 20, 30];
    let p_a = &mut a as *mut _ as usize;
    let p_ref_a = &&mut a as *const _ as usize;

    // Check types using pointer differences
    if p_a != p_ref_a {
        std::process::exit(1);
    }
    if p_a == p_ref_a {
        std::process::exit(2);
    }

    // Array element checks
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Size and alignment checks
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = ['a', 'b', 'c', '\0'];

    // String literal type check
    if "abc".as_ptr() as usize == s.as_ptr() as usize {
        std::process::exit(8);
    }

    // Character checks
    if s[0] != 'a' {
        std::process::exit(9);
    }
    if s[1] != 'b' {
        std::process::exit(10);
    }
    if s[2] != 'c' {
        std::process::exit(11);
    }
    if s[3] != '\0' {
        std::process::exit(12);
    }
}