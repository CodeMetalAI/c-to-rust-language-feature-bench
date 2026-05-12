fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S {
        a: 7,
        b: 9.5,
    };

    let pi = &xi;
    let pd = &xd;
    let ps = &xs;

    let v1: *const i32 = pi as *const i32;
    let v2: *const f64 = pd as *const f64;
    let v3: *const S = ps as *const S;

    let pi2 = unsafe { &*v1 };
    let pd2 = unsafe { &*v2 };
    let ps2 = unsafe { &*v3 };

    assert_eq!(pi2, pi);
    assert_eq!(pd2, pd);
    assert_eq!(ps2, ps);

    assert_eq!(*pi2, 42);
    assert_eq!(*pd2, 3.25);
    assert_eq!(ps2.a, 7);
    assert_eq!(ps2.b, 9.5);

    let v1b = pi2 as *const i32;
    let v2b = pd2 as *const f64;
    let v3b = ps2 as *const S;

    assert_eq!(v1b, v1);
    assert_eq!(v2b, v2);
    assert_eq!(v3b, v3);
}

struct S {
    a: i32,
    b: f64,
}