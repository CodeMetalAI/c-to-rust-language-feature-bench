fn main() {
    let w = [struct {
        a: [i32; 3],
        b: i32,
    } {
        a: [1, 0, 0],
        b: 0,
    },
    struct {
        a: [2, 0, 0],
        b: 0,
    }];

    assert_eq!(w.len(), 2);
    assert_eq!(w[0].a[0], 1);
    assert_eq!(w[0].a[1], 0);
    assert_eq!(w[0].a[2], 0);
    assert_eq!(w[0].b, 0);

    assert_eq!(w[1].a[0], 2);
    assert_eq!(w[1].a[1], 0);
    assert_eq!(w[1].a[2], 0);
    assert_eq!(w[1].b, 0);

    assert!(true);
}