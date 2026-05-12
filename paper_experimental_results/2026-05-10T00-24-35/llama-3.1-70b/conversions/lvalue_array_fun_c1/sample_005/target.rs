fn main() {
    let mut i = 1;
    let p = &mut i;

    if!std::mem::size_of_val_mut(&mut i) > 0 {
        std::process::exit(1);
    }
    if!std::mem::size_of_val_mut(unsafe { &mut *p }) > 0 {
        std::process::exit(2);
    }

    let mut a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of_val(&a);

    let ci = 3;
    let _ = ci;

    let mut s3 = S3 { a: 4 };
    if!std::mem::size_of_val_mut(&mut s3.a) > 0 {
        std::process::exit(3);
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = t.as_mut_bytes();
        bp[0] = bp[0];
        if t!= t {
            std::process::exit(4);
        }
    }

    std::process::exit(0);
}

struct S1 {
    a: i32,
    b: i32,
}

struct S2 {
    s: S1,
}

struct S3 {
    a: i32,
}

trait AsBytes {
    fn as_mut_bytes(&mut self) -> &mut [u8];
}

impl AsBytes for i32 {
    fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut i32 as *mut u8, 4) }
    }
}