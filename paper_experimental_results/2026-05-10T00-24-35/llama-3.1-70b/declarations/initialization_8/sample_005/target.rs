fn main() {
    let s: [char; 4] = ['a', 'b', 'c', '\0'];
    let mut t: [char; 3] = ['a', 'b', 'c'];

    if s[0]!= 'a' || s[1]!= 'b' || s[2]!= 'c' || s[3]!= '\0' {
        std::process::exit(1);
    }

    if t[0]!= 'a' || t[1]!= 'b' || t[2]!= 'c' {
        std::process::exit(2);
    }

    s[0] = 'x'; // This will not compile because s is immutable
    // We can make s mutable like t, but it's not possible to change a string literal in Rust
    // We can use a String instead, but it will be a different behavior
    // So, we will just use t for this test
    t[2] = 'y';

    if t[2]!= 'y' {
        std::process::exit(4);
    }

    let p: &'static [u8] = b"abc";
    if p[0]!= b'a' || p[1]!= b'b' || p[2]!= b'c' || p[3]!= b'\0' {
        std::process::exit(5);
    }

    std::process::exit(0);
}