struct V {
    i: i32,
    j: i32,
    m: i32,
}

impl V {
    fn new() -> Self {
        V { i: 0, j: 0, m: 0 }
    }
}

fn main() {
    let mut v1 = V::new();
    v1.i = 2;
    unsafe {
        let k_ptr: *mut i32 = &mut v1.i;
        let k_ptr = k_ptr as *mut i64;
        *k_ptr = 5;
    }

    if v1.i != 2 {
        std::process::exit(1);
    }

    unsafe {
        let k_ptr: *mut i32 = &mut v1.i;
        let k_ptr = k_ptr as *mut i64;
        if *k_ptr != 5 {
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}