fn main() {
    let mut x = 0;

    {
        // This block is never executed because of the goto statement.
        // The compiler may optimize it away, but we keep it for
        // clarity and to match the C++ code's structure.
        goto end;
        x = 1;
    }

    end:
    x += 1;
    if x == 1 {
        0
    } else {
        1
    }
}