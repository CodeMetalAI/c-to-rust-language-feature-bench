fn main() {
    static mut SINK: i32 = 0;

    fn copyt(n: i32) {
        let n = n + 1;

        // This is tricky to do safely in Rust.  We need to create a fixed-size array
        // *at runtime* based on a value passed in.  This is not possible in safe Rust
        // without resorting to unsafe code.  The original code uses a Variable Length Array (VLA),
        // which is not supported in Rust.
        // We will use a Vec instead, and then manually check the size conditions.

        let mut a = Vec::with_capacity((n - 1) as usize);
        let mut b: Vec<i32> = Vec::with_capacity(n as usize);

        let mut i = 0;
        while i < n {
            b.push(1000 + i);
            i += 1;
        }

        i = 1;
        while i < n {
            a.push(b[i as usize]);
            i += 1;
        }

        if (a.len() as i32) != (n - 1) {
            unsafe { SINK = 1 };
        }
        if (b.len() as i32) != n {
            unsafe { SINK = 2 };
        }

        if a.len() > 0 && a[0] != 1001 {
            unsafe { SINK = 3 };
        }
        if a.len() > 1 && a[(n - 2) as usize] != 1000 + (n - 1) {
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