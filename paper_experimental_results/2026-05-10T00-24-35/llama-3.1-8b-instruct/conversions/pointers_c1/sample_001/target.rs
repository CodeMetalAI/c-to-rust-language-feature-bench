fn main() {
    let xi: i32 = 42;
    let xd: f64 = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let pi = &xi;
    let pd = &xd;
    let ps = &xs;

    let v1 = pi as *const i32;
    let v2 = pd as *const f64;
    let v3 = ps as *const S;

    let pi2 = v1 as *const i32;
    let pd2 = v2 as *const f64;
    let ps2 = v3 as *const S;

    assert_eq!(pi2, &xi);
    assert_eq!(pd2, &xd);
    assert_eq!(ps2, &xs);

    assert_eq!(*pi2, 42);
    assert_eq!(*pd2, 3.25);
    assert_eq!(ps2.a, 7);
    assert_eq!(ps2.b, 9.5);

    let v1b = pi2 as *const i32 as *const ();
    let v2b = pd2 as *const f64 as *const ();
    let v3b = ps2 as *const S as *const ();

    assert_eq!(v1b, v1);
    assert_eq!(v2b, v2);
    assert_eq!(v3b, v3);
}