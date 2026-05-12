fn main() {
    const X: i32 = 3;
    const F: fn(i32) -> i32 = |a: i32| a + X;
    const T: fn(fn(i32) -> i32) -> fn(i32) -> i32 = |a: fn(i32) -> i32| a;
    const G: fn(i32) -> i32 = F;
    const Q: fn(i32) -> i32 = |x: i32| x;
    const P: fn() -> i32 = || 0;
    const R: fn(i32, i32) -> i32 = |x: i32, y: i32| x * 10 + y;
    const STR: fn(&str) -> String = |x: &str| x.to_string();

    let v = F(10);
    let u = F((3, 4));
    let w = T(G)(0);

    let i: [i32; 4] = [Q(1), R(2, 3), R(4, 0), R(0, 5), R(0, 0)];
    let c: [[char; 6]; 2] = [['h', 'e', 'l', 'l', 'o', '\0'], ['\0', '\0', '\0', '\0', '\0', '\0']];

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

    if c[0][0] != 'h' {
        std::process::exit(8);
    }
    if c[0][1] != 'e' {
        std::process::exit(9);
    }
    if c[0][2] != 'l' {
        std::process::exit(10);
    }
    if c[0][3] != 'l' {
        std::process::exit(11);
    }
    if c[0][4] != 'o' {
        std::process::exit(12);
    }
    if c[0][5] != '\0' {
        std::process::exit(13);
    }

    if c[1][0] != '\0' {
        std::process::exit(14);
    }
}