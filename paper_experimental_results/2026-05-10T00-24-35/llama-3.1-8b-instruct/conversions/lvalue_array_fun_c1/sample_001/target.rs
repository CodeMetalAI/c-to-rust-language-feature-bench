/// Equivalent to lvalue_array_fun_c1.c.
fn main() -> i32 {
    // Equivalent to typedef unsigned long size_t;
    type Size = usize;

    macro_rules! ok_mod {
        ($x:expr) => {{
            let _ = std::mem::size_of_val(&$x);
            _ > 0
        }};
    }

    // Equivalent to struct S1 { int a; const int b; };
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct S1 {
        a: i32,
        b: i32,
    }

    // Equivalent to struct S2 { struct S1 s; };
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct S2 {
        s: S1,
    }

    // Equivalent to struct S3 { int a; };
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct S3 {
        a: i32,
    }

    // Equivalent to main(void)
    if !ok_mod(1) {
        return 1;
    }
    if !ok_mod(&1) {
        return 2;
    }

    // Equivalent to int a[2] = {1, 2};
    let arr: [i32; 2] = [1, 2];

    // Equivalent to (void)a;
    let _ = arr;

    // Equivalent to (void)sizeof(a);
    let _ = std::mem::size_of_val(&arr);

    // Equivalent to const int ci = 3;
    let ci = 3;

    // Equivalent to (void)ci;
    let _ = ci;

    // Equivalent to struct S3 s3 = {4};
    let s3 = S3 { a: 4 };

    // Equivalent to if (!OK_MOD(s3.a)) return 3;
    if !ok_mod(s3.a) {
        return 3;
    }

    // Equivalent to struct S1 s1 = {5, 6};
    let mut s1 = S1 { a: 5, b: 6 };

    // Equivalent to struct S2 s2 = {{7, 8}};
    let mut s2 = S2 {
        s: S1 {
            a: 7,
            b: 8,
        },
    };

    // Equivalent to (void)s1;
    let _ = s1;

    // Equivalent to (void)s2;
    let _ = s2;

    {
        // Equivalent to {
        let mut t: i32 = 9;
        // Equivalent to unsigned char *bp = (unsigned char *)&t;
        let bp: *const u8 = t as *const _;

        // Equivalent to bp[0] = bp[0];
        unsafe { *bp = *bp; }

        // Equivalent to t = t;
        t = t;

        // Equivalent to if (t != t) return 4;
        if t != t {
            return 4;
        }
    }

    // Equivalent to return 0;
    0
}