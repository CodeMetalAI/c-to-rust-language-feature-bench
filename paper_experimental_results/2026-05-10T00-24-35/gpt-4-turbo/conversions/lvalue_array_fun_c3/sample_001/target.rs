fn main() {
    let mut a: [i32; 3] = [10, 20, 30];

    let a_ptr = &mut a as *mut i32;
    let a_ref = &mut a;

    if std::mem::size_of_val(&*a_ptr) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::size_of_val(&a_ref) != std::mem::size_of::<[i32; 3]>() {
        std::process::exit(2);
    }

    if unsafe { *a_ptr } != 10 {
        std::process::exit(3);
    }
    if unsafe { *a_ptr.add(1) } != 20 {
        std::process::exit(4);
    }
    if unsafe { *a_ptr.add(2) } != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = ['a', 'b', 'c', '\0'];

    if std::mem::size_of_val(&"abc") != std::mem::size_of::<&str>() {
        std::process::exit(8);
    }
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