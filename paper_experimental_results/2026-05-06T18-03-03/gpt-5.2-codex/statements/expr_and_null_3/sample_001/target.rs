fn main() -> i32 {
    let mut x = 0;

    {
        'end: loop {
            break 'end;
            x = 1;
        }
    }

    x += 1;
    if x == 1 { 0 } else { 1 }
}