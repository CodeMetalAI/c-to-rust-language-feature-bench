fn main() {
    const X1: i32 = 3;
    fn f1(a: i32) -> i32 { a + X1 }
    const X2: i32 = 2;
    fn f2(a: i32) -> i32 { a + X2 }

    let v = f2(10);
    let u = f2((3, 4).1);
    let w = f2(0);

    let i = [1, 23, 4, 5, 0];

    let c = [["hello".to_string(), "".to_string()]];

    if v != 12 {
        std::process::exit(1);
    }
    if u != 6 {
        std::process::exit(2);
    }
    if w != 2 {
        std::process::exit(3);
    }

    if i[0] != 1 {
        std::process::exit(4);
    }
    if i[1] != 23 {
        std::process::exit(5);
    }
    if i[2] != 4 {
        std::process::exit(6);
    }
    if i[3] != 5 {
        std::process::exit(7);
    }

    let hello_chars = c[0][0].chars().collect::<Vec<char>>();
    let empty_chars = c[0][1].chars().collect::<Vec<char>>();

    if hello_chars[0] != 'h' {
        std::process::exit(8);
    }
    if hello_chars[1] != 'e' {
        std::process::exit(9);
    }
    if hello_chars[2] != 'l' {
        std::process::exit(10);
    }
    if hello_chars[3] != 'l' {
        std::process::exit(11);
    }
    if hello_chars[4] != 'o' {
        std::process::exit(12);
    }
    if hello_chars.get(5).is_some() {
        std::process::exit(13);
    }

    if !empty_chars.is_empty() {
        std::process::exit(14);
    }

    std::process::exit(0);
}