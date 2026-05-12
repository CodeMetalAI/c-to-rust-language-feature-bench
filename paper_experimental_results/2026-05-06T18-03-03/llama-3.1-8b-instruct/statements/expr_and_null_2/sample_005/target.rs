fn main() {
    let s = "12345";
    let mut acc = 0;
    for c in s.chars() {
        if c != '\0' {
            acc += 1;
        }
    }
    println!("{}", acc == 5);
}