fn main() {
    static mut SINK: i32 = 0;

    fn copyt(n: i32) {
        let n = n + 1;

        // Create a vector of size n.  This is safe because n is known at runtime.
        let mut b: Vec<i32> = vec![0; n as usize];

        let mut i = 0;
        while i < n {
            b[i as usize] = 1000 + i;
            i += 1;
        }

        i = 1;
        let mut a: Vec<i32> = vec![0; (n - 1) as usize];
        while i < n {
            a[(i - 1) as usize] = b[i as usize];
            i += 1;
        }

        if (a.len() as i32) != (n - 1) {
            unsafe { SINK = 1 };
        }
        if (b.len() as i32) != n {
            unsafe { SINK = 2 };
        }

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