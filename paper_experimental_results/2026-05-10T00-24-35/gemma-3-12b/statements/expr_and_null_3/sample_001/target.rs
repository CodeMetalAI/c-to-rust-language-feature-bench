fn main() {
    let mut x = 0;

    {
        // The goto statement is replaced with a simple block that is never executed.
    }

    x += 1;
    if x == 1 {
        0
    } else {
        1
    }
}