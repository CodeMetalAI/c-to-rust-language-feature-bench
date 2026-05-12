fn main() -> i32 {
    let c: char = 'A';
    let mut p: Option<&char> = None;

    p = Some(&c);
    
    if c != 'A' {
        return 1;
    }

    0
}