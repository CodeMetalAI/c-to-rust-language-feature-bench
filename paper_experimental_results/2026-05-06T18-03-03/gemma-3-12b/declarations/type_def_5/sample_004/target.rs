fn main() {
    static mut SINK: i32 = 0;

    fn copyt(n: i32) {
        let n = n + 1;

        // Create a vector of size n.  This is equivalent to the C++ array 'b'.
        let mut b: Vec<i32> = vec![0; n as usize];

        // Initialize the vector 'b'.
        for i in 0..n {
            b[i as usize] = 1000 + i;
        }

        // Create a vector of size n-1. This is equivalent to the C++ array 'a'.
        let mut a: Vec<i32> = vec![0; (n - 1) as usize];

        // Copy elements from 'b' to 'a'.
        for i in 1..n {
            a[(i - 1) as usize] = b[i as usize];
        }

        // Check sizes and set sink if necessary.
        if (a.len() as i32) != (n - 1) {
            unsafe { SINK = 1 };
        }
        if (b.len() as i32) != n {
            unsafe { SINK = 2 };
        }

        // Check values and set sink if necessary.
        if a[0] != 1001 {
            unsafe { SINK = 3 };
        }
        if a[(n - 2) as usize] != 1000 + (n - 1) {
            unsafe { SINK = 4 };
        }
    }

    unsafe { SINK = 0 };
    copyt(7);

    if unsafe { SINK } != 0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}