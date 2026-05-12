fn main() {
    let mut x = 0;

    {
        if false { // This simulates the `goto end;` behavior by making the assignment unreachable
            x = 1;
        }
    }

    x += 1;
    std::process::exit(if x == 1 { 0 } else { 1 });
}