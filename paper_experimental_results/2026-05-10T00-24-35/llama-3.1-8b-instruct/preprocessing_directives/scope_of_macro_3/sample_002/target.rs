use std::fmt;

const X: u32 = 3;
const F: fn(u32) -> u32 = |x| x + X;
const G: fn(u32) -> u32 = F;
const X_2: u32 = 2;
const P: fn() -> String = || "int".to_string();
const R: fn(&str, &str) -> String = move |a, b| format!("{}{}", a, b);
const Q: fn(&str) -> String = |x| x.to_string();
const T: fn(&fn(u32) -> u32, u32) -> u32 = |a, x| a(x);
const STR: fn(&str) -> String = |x| format!("{}", x);

fn main() {
    let v = F(10);
    let u = F(3 + 4);
    let w = T(&G, 0);

    let arr = [
        1,
        R("2", "3"),
        R("4", ""),
        R("", "5"),
        R("", ""),
    ];

    let mut c: [[u8; 6]; 2] = [[0; 6]; 2];

    c[0][..str("hello").into_bytes().len()]
        .clone_from_slice(str("hello").into_bytes());

    if v != 12 {
        eprintln!("v != 12");
        return;
    }
    if u != 6 {
        eprintln!("u != 6");
        return;
    }
    if w != 2 {
        eprintln!("w != 2");
        return;
    }

    if arr[0] != 1 {
        eprintln!("arr[0] != 1");
        return;
    }
    if arr[1] != 23 {
        eprintln!("arr[1] != 23");
        return;
    }
    if arr[2] != 4 {
        eprintln!("arr[2] != 4");
        return;
    }
    if arr[3] != 5 {
        eprintln!("arr[3] != 5");
        return;
    }

    if c[0][0] != b'h' {
        eprintln!("c[0][0] != b'h'");
        return;
    }
    if c[0][1] != b'e' {
        eprintln!("c[0][1] != b'e'");
        return;
    }
    if c[0][2] != b'l' {
        eprintln!("c[0][2] != b'l'");
        return;
    }
    if c[0][3] != b'l' {
        eprintln!("c[0][3] != b'l'");
        return;
    }
    if c[0][4] != b'o' {
        eprintln!("c[0][4] != b'o'");
        return;
    }
    if c[0][5] != b'\0' {
        eprintln!("c[0][5] != b'\\0'");
        return;
    }

    if c[1][0] != b'\0' {
        eprintln!("c[1][0] != b'\\0'");
        return;
    }

    println!("0");
}