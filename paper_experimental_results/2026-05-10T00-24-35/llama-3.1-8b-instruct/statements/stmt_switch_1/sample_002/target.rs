fn main() -> i32 {
    let mut expr = 0;
    let mut i = 4;

    match expr {
        0 => i = 17,
    }

    (i == 17) as i32
}