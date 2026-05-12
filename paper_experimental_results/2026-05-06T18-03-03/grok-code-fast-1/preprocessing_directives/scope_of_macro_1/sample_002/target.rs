fn main() -> i32 {
    let a: [i32; 100] = [0; 100];
    if a.len() != 100 {
        return 1;
    }
    let mut tabsiz = 0;
    tabsiz = 7;
    if tabsiz != 7 {
        return 2;
    }
    0
}