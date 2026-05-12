fn main() -> i32 {
    let c = 'A';
    let mut p: Option<&char> = None;

    p = Some(&c);
    
    if c != 'A' {
        return 1;
    }

    return 0;
}