fn main() {
    static mut SINK: i32 = 0;

    fn copyt(n: i32) {
        let n = n + 1;
        let a: [i32; n as usize] = [0; n as usize];
        let mut b: Vec<i32> = Vec::with_capacity(n as usize);

        for i in 0..n {
            b.push(1000 + i);
        }

        for i in 1..n {
            a[(i - 1) as usize] = b[i as usize];
        }

        if (a.len() as i32) != (n - 1) {
            unsafe { SINK = 1 };
        }
        if (b.len() as i32) != n {
            unsafe { SINK = 2 };
        }

        if unsafe { a[0] } != 1001 {
            unsafe { SINK = 3 };
        }
        if unsafe { a[(n - 2) as usize] } != 1000 + (n - 1) {
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