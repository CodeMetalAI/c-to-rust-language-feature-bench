fn main() -> i32 {
    let expr = 0;
    let mut i = 4;

    match expr {
        0 => {
            i = 17;
            if i == 17 {
                return 0;
            } else {
                return 1;
            }
        }
        _ => {}
    }

    1 // Default return value if no case matches
}