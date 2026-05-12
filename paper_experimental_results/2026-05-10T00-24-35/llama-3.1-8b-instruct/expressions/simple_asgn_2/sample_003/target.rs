fn main() -> i32 {
    let mut c: u8 = 0;
    let mut i: i32 = 100;
    let mut l: i64 = 0;

    l = c = i as u8;

    if c!= i as u8 {
        1
    } else if l!= c as i64 {
        2
    } else if l!= i as i64 {
        3
    } else {
        0
    }
}