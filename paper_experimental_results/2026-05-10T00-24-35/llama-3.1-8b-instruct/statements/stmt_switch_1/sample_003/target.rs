fn main() -> i32 {
    let mut i: i32 = 0;

    match i {
        0 => i = 17,
        _ => {}
    };

    return if i == 17 { 0 } else { 1 };
}