fn main() {
    static mut SINK: i32 = 0;

    fn copyt(n: i32) {
        let n_plus_one = n + 1;

        // Create a vector of size n_plus_one. This acts as 'a' in C.
        let mut a: Vec<i32> = Vec::with_capacity(n_plus_one as usize);
        
        // Create a vector of size n_plus_one. This acts as 'b' in C.
        let mut b: Vec<i32> = Vec::with_capacity(n_plus_one as usize);

        let mut i = 0;
        while i < n_plus_one {
            b.push(1000 + i);
            i += 1;
        }

        i = 1;
        while i < n_plus_one {
            a.push(b[i as usize]);
            i += 1;
        }

        if (a.len() as i32) != (n_plus_one - 1) {
            unsafe { SINK = 1 };
        }
        if (b.len() as i32) != n_plus_one {
            unsafe { SINK = 2 };
        }

        if unsafe { a[0] } != 1001 {
            unsafe { SINK = 3 };
        }
        if unsafe { a[(n_plus_one - 2) as usize] } != 1000 + (n_plus_one - 1) {
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