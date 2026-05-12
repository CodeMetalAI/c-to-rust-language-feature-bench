fn main() {
    let mut x = 0;
    {
        break;
        x = 1;
    }
    x += 1;
    assert!(x == 1);
}