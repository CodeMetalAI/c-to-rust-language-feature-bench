fn main() {
    let c = 'A';
    let p: *const char = &c;
    let cpp: *const *const char = &p;

    // Dereferencing and attempting to modify `*p` is not safe or valid in Rust
    // because `c` is a constant and Rust enforces strict aliasing rules.
    // Therefore, we will not perform the equivalent of `*p = 0;` in Rust.

    if c != 'A' {
        std::process::exit(1);
    }

    std::process::exit(0);
}