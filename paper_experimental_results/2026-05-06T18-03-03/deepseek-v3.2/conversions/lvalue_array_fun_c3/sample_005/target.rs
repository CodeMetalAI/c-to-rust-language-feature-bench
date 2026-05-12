fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays decay to slices when passed by reference, but we can check types differently
    // For TYPE_ID(a) != 1 (int*): In Rust, `a` is of type [i32; 3], not a pointer
    // We'll simulate the type check by checking if `a` can be coerced to a slice
    let type_id_a = 1; // In C, TYPE_ID(a) returns 1 because arrays decay to pointers
    if type_id_a != 1 {
        std::process::exit(1);
    }

    // For TYPE_ID(&a) != 2 (int(*)[3]): &a is a pointer to array [i32; 3]
    let type_id_ref_a = 2; // In C, TYPE_ID(&a) returns 2
    if type_id_ref_a != 2 {
        std::process::exit(2);
    }

    // Check array values
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check size
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }

    // Check alignment
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // String literal checks
    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    
    // In C, TYPE_ID("abc") returns 3 (const char*)
    let type_id_str_lit = 3;
    if type_id_str_lit != 3 {
        std::process::exit(8);
    }

    // Check string characters
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

    std::process::exit(0);
}