fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays coerce to slices, but we can check their types
    // TYPE_ID(a) != 1: In C, array decays to int*, so TYPE_ID is 1
    // Rust doesn't have _Generic, so we'll check if it's a reference to array
    // Actually, we can't directly check types like C's _Generic, so we'll 
    // verify the behavior instead
    
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

    // Check size and alignment
    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    
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