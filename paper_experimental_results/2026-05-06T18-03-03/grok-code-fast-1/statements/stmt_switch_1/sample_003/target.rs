fn main() -> i32 {
    let expr = 0;
    let mut i: i32;
    i = 4;
    match expr {
        0 => {
            i = 17;
            if i == 17 { 0 } else { 1 }
        }
        _ => 0,
    }
}