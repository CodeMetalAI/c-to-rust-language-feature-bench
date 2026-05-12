fn main() -> i32 {
    let c: i32 = 100;
    let i = c;
    let l = c as i64;

    if c!= c as i8 {
        1
    } else if l!= c as i64 {
        2
    } else if l!= i as i64 {
        3
    } else {
        0
    }
}