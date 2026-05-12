fn main() -> i32 {
    let expr = 0;
    let mut i = 4;

    match expr {
        0 => {
            i = 17;
            return if i == 17 { 0 } else { 1 };
        }
        _ => {}
    }
    
    1 // This line is unreachable but needed to satisfy the return type
}