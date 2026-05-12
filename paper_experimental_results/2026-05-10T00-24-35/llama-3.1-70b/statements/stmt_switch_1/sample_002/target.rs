fn main() -> i32 {
    let mut i = 4;
    match 0 {
        0 => {
            i = 17;
            if i == 17 { 0 } else { 1 }
        }
        _ => unreachable!(),
    }
}