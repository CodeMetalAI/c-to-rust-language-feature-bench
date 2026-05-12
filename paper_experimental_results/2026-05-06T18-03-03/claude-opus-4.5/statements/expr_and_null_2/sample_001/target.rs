fn main() {
    let s = "12345";
    let mut acc = 0;
    let bytes = s.as_bytes();
    let mut i = 0;
    
    while i < bytes.len() {
        let c = bytes[i];
        i += 1;
        if c == b'\0' {
            break;
        }
        acc += 1;
    }
    
    std::process::exit(if acc == 5 { 0 } else { 1 });
}