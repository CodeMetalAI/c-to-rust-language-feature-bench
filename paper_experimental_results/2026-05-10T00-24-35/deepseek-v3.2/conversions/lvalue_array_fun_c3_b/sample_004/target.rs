fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // TYPE_ID(a) should be int* -> 1
    // In Rust, array name doesn't decay to pointer automatically
    // We'll check the type by pattern matching
    let type_id_a = match std::any::type_name_of_val(&a) {
        s if s.contains("[i32; 3]") => 1,  // Array type
        _ => 99,
    };
    if type_id_a != 1 {
        std::process::exit(1);
    }

    // TYPE_ID(&a) should be int (*)[3] -> 2
    let type_id_ref_a = match std::any::type_name_of_val(&&a) {
        s if s.contains("&[i32; 3]") => 2,  // Reference to array
        _ => 99,
    };
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

    // String literal and array
    let s: [char; 4] = ['a', 'b', 'c', '\0'];

    // TYPE_ID("abc") should be const char* -> 3
    // In Rust, string literal is &'static str
    let type_id_str = match std::any::type_name::<&'static str>() {
        s if s.contains("&str") => 3,
        _ => 99,
    };
    if type_id_str != 3 {
        std::process::exit(8);
    }

    // Check string array values
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