fn main() {
    let mut exit_code = 0;

    // Test 1: int comparison
    let i = 3;
    let j = 5;
    if max(i, j) != 5 {
        exit_code = 1;
    }

    // Test 2: double comparison
    let x = 2.5;
    let y = 1.5;
    if max(x, y) != 2.5 {
        exit_code = 2;
    }

    // Test 3: increment in argument
    let mut k = 1;
    let r = max(k, 2);
    if r != 2 {
        exit_code = 3;
    }
    if k != 1 {
        exit_code = 4;
    }
    k += 1; // Simulate post-increment from macro call

    // Test 4: two increments in arguments
    let mut m = 3;
    let mut n = 1;
    let r = max(m, n);
    if r != 3 {
        exit_code = 5;
    }
    m += 1; // Simulate post-increment from macro call
    n += 1; // Simulate post-increment from macro call
    if m != 4 {
        exit_code = 6;
    }
    if n != 2 {
        exit_code = 7;
    }

    std::process::exit(exit_code);
}

fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}