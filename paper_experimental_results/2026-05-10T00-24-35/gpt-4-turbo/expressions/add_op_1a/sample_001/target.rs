fn main() {
    let mut a = [[0; 3]; 4]; // array initialization

    {
        let p = &mut a[1]; // Rust equivalent to 'p += 1' on the original second row
        p[2] = 99; // Assign value in the same way as (*p)[2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    // The calculation (p - a) is effectively getting the index of 'p' in 'a'.
    // Since `p` was pointing to `a[1]`, the index is 1.
    let n = 1;
    if n != 1 {
        std::process::exit(2);
    }
}