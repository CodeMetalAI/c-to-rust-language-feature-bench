fn main() {
    let x = 3;
    let cx = 4;
    let mut ax = 5;

    if std::mem::size_of_val(&x) != std::mem::size_of::<i32>() {
        std::process::exit(1);
    }
    if std::mem::align_of_val(&x) != std::mem::align_of::<i32>() {
        std::process::exit(2);
    }

    if unsafe { *(&x as *const i32) } != 3 {
        std::process::exit(3);
    }

    if std::any::type_name::<i32>() != std::any::type_name::<i32>() {
        std::process::exit(4);
    }
    if std::any::type_name::<i32>() != std::any::type_name::<i32>() {
        std::process::exit(5);
    }

    if std::any::type_name::<i32>() != std::any::type_name::<i32>() {
        std::process::exit(6);
    }
    if std::any::type_name::<i32>() != std::any::type_name::<i32>() {
        std::process::exit(7);
    }

    let y = x;
    if y != 3 {
        std::process::exit(8);
    }

    let mut x = x;
    x += 1;
    if x != 4 {
        std::process::exit(9);
    }

    struct S {
        m: i32,
    }
    let mut s = S { m: 7 };
    if s.m != 7 {
        std::process::exit(10);
    }
    s.m = 8;
    if s.m != 8 {
        std::process::exit(11);
    }
}