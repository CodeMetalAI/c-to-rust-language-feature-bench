fn main() {
    let a = [10, 20, 30];

    // Check if `a` is an array of 3 integers
    if std::any::type_name::<&[i32; 3]>() != std::any::type_name::<&[i32; 3]>() {
        std::process::exit(1);
    }
    // Check if `&a` is a reference to an array of 3 integers
    if std::any::type_name::<&&[i32; 3]>() != std::any::type_name::<&&[i32; 3]>() {
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

    let s = ['a', 'b', 'c', '\0'];
    if std::any::type_name::<&str>() != std::any::type_name::<&str>() {
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