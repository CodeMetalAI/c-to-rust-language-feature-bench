const fn ok_mod<T: Copy>(x: T) -> bool {
    let _ = x;
    std::mem::size_of::<T>() > 0
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct S1 {
    a: i32,
    b: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct S2 {
    s: S1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct S3 {
    a: i32,
}

fn main() {
    let mut i = 1;
    let p = &mut i;

    if !ok_mod(i) {
        panic!("lvalue_array_fun_c1: i is not an lvalue");
    }
    if !ok_mod(*p) {
        panic!("lvalue_array_fun_c1: *p is not an lvalue");
    }

    let a = [1, 2];
    let _ = a;
    let _ = std::mem::size_of(&a);

    let ci = 3;
    let _ = ci;

    let s3 = S3 { a: 4 };
    if !ok_mod(s3.a) {
        panic!("lvalue_array_fun_c1: s3.a is not an lvalue");
    }

    let s1 = S1 { a: 5, b: 6 };
    let s2 = S2 { s: S1 { a: 7, b: 8 } };
    let _ = s1;
    let _ = s2;

    {
        let mut t = 9;
        let bp = t as *mut u8;
        unsafe {
            *bp = *bp;
        }
        t = t;
        if t != t {
            panic!("lvalue_array_fun_c1: t is not an lvalue");
        }
    }

    assert_eq!(0, 0);
}