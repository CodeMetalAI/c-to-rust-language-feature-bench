fn main() {
    let mut xi: i32 = 42;
    let mut xd: f64 = 3.25;
    let mut xs = S { a: 7, b: 9.5 };

    let pi = &mut xi;
    let pd = &mut xd;
    let ps = &mut xs;

    let v1 = pi as *mut i32;
    let v2 = pd as *mut f64;
    let v3 = ps as *mut S;

    let pi2 = unsafe { &*v1 };
    let pd2 = unsafe { &*v2 };
    let ps2 = unsafe { &*v3 };

    assert_eq!(pi2, &mut xi);
    assert_eq!(pd2, &mut xd);
    assert_eq!(ps2, &mut xs);

    assert_eq!(*pi2, 42);
    assert_eq!(*pd2, 3.25);
    assert_eq!(ps2.a, 7);
    assert_eq!(ps2.b, 9.5);

    let v1b = pi2 as *const i32;
    let v2b = pd2 as *const f64;
    let v3b = ps2 as *const S;

    assert_eq!(v1b, v1 as *const i32);
    assert_eq!(v2b, v2 as *const f64);
    assert_eq!(v3b, v3 as *const S);
}

struct S {
    a: i32,
    b: f64,
}