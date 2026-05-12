fn main() {
    let expr = 0;

    let mut i = 4;
    match expr {
        0 => {
            i = 17;
            return (i == 17) as i32;
        }
    }
    return 1;
}