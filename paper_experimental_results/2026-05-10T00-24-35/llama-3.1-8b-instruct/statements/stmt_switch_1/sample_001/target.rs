fn main() -> i32 {
    let mut expr = 0;
    let mut i = 0;

    match expr {
        0 => {
            i = 17;
        }
        _ => {
            i = 4;
        }
    }

    if i == 17 {
        0
    } else {
        1
    }
}