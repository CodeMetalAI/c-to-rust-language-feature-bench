fn main() {
    let mut x = 5;

    let y = x - 1;
    assert_eq!(y, 4);
    assert_eq!(x, 4);

    x = 10;
    let y = x - 1;
    let z = 10;
    let z = z - 1;
    assert_eq!(y, 9);
    assert_eq!(x, 9);
    assert_eq!(z, 9);

    assert_eq!(0, 0);
}