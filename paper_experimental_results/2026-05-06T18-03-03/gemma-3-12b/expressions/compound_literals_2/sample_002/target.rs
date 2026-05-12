fn f(init: i32) -> i32 {
    let mut x = init;
    let mut p = &mut x;

    // Create a mutable array of two i32s on the stack.
    // The first element is initialized with the value of p (which is x).
    // The second element is initialized to 0.
    let arr = [(*p), 0];
    *p = arr[0]; // Assign the first element to p.
    
    if arr[0] != init {
        return 1;
    }
    if arr[1] != 0 {
        return 2;
    }

    0
}

fn main() {
    if f(7) != 0 {
        std::process::exit(10);
    }
    if f(-3) != 0 {
        std::process::exit(11);
    }
    std::process::exit(0);
}