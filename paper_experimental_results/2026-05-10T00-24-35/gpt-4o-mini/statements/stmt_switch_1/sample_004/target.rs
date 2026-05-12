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
        },
        _ => {}
    }

    // This return statement is here to satisfy the return type,
    // but it should never be reached given the defined cases.
    1
}