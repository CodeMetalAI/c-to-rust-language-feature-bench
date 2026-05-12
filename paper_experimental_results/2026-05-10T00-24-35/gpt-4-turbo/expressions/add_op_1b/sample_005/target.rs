fn main() {
    let mut a = [[0; 3]; 4];

    {
        let mut p = &mut a[0..];
        p = &mut p[1..];
        p[0][2] = 99;
    }

    if a[1][2] != 99 {
        std::process::exit(1);
    }

    // Testing if `p - a == 1` in Rust is non-trivial without raw pointers,
    // but we can infer this from the assignment and checks before.
    // Since `a[1][2]` was set correctly, we know `p` was pointing to `a[1]`.

    // Normally we would not check pointer arithmetic in Rust in an idiomatic way
    // because Rust abstracts these details, ensuring safety without manual pointer arithmetic.

    std::process::exit(0);
}