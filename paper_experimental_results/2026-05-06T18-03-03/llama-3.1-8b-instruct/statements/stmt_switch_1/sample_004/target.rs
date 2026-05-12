fn main() {
    let mut expr = 0;

    let mut i = 4;
    match expr {
        0 => i = 17,
        _ => (),
    }
    assert_eq!(i, 17);
    assert!(i == 17);
}