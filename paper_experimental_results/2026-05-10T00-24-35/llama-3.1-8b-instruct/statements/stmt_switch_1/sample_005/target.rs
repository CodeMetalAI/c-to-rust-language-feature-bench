fn main() {
    let mut expr = 0;

    let mut i = match expr {
        0 => {
            i = 4;
            0
        }
        _ => 0,
    };

    i = 17;
    assert_eq!(i, 17);
    assert!(i == 17);
    println!("{}", i == 17);
}