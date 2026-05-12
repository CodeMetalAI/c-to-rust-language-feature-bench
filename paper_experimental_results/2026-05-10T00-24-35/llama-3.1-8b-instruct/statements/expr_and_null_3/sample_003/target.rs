fn main() {
    let mut x = 0;

    {
        let _ = ();
        x = 1;
    }

    x += 1;
    assert!(x == 1);
}