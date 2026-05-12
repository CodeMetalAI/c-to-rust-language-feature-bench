fn main() {
    static mut SINK: i32 = 0;

    fn copyt(n: i32) {
        let n = n + 1;

        // Create a vector of size n.  This is equivalent to the C++ B[n]
        let b: Vec<i32> = (0..n).map(|i| 1000 + i).collect();

        // Create a vector of size n-1. This is equivalent to the C++ a[n]
        let a: Vec<i32> = (0..n - 1).map(|_| 0).collect();

        unsafe {
            let mut i = 1;
            while i < n {
                a[i - 1] = b[i];
                i += 1;
            }

            if (a.len() as i32) != (n - 1) {
                SINK = 1;
            }
            if (b.len() as i32) != n {
                SINK = 2;
            }

            if a[0] != 1001 {
                SINK = 3;
            }
            if a[n - 2] != 1000 + (n - 1) {
                SINK = 4;
            }
        }
    }

    unsafe {
        SINK = 0;
        copyt(7);
        if SINK != 0 {
            std::process::exit(1);
        }
    }
}