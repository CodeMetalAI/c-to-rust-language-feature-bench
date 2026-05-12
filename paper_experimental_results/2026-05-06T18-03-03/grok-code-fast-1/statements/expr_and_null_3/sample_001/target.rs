fn main() -> i32 {
    let mut x = 0;
    // The goto skips the x = 1 assignment
    x += 1;
    if x == 1 { 0 } else { 1 }
}