fn main() {
    let a: [i32; 3] = [10, 20, 30];
    let a_ref = &a;
    let a_ptr = &a[0] as *const i32;

    if std::mem::size_of_val(&*a_ptr) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::size_of_val(a_ref) != std::mem::size_of_val(&[1i32, 2, 3]) {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s = "abc";
    if std::mem::size_of_val(&s) != std::mem::size_of_val(&"def") {
        std::process::exit(8);
    }
    if s.as_bytes()[0] != b'a' {
        std::process::exit(9);
    }
    if s.as_bytes()[1] != b'b' {
        std::process::exit(10);
    }
    if s.as_bytes()[2] != b'c' {
        std::process::exit(11);
    }
    if s.as_bytes().len() != 3 {
        std::process::exit(12);
    }
}