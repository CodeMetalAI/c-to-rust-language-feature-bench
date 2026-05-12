fn main() {
    // Macros are tricky, let's try to replicate their behavior
    let x1 = 3;
    let f1 = |a: i32| (a + x1);
    let x2 = 2;
    let f2 = |a: i32| (a + x2);

    let t = |a: i32| a;
    let g = f2;
    let q = |x: i32| x;
    let p = || Vec<i32>;
    let r = |x: i32, y: i32| x * 10 + y;
    let str = |x: &str| format!("\"{}\"", x);

    let v = f1(10);
    let u = f1(3);
    let w = t(g)(0);

    let i: Vec<i32> = vec![q(1), r(2, 3), r(4, 0), r(0, 5), r(0, 0)];
    let c: [[char; 6]; 2] = [
        str("hello").chars().collect::<Vec<char>>().try_into().unwrap(),
        vec!['\0', '\0', '\0', '\0', '\0', '\0'].try_into().unwrap(),
    ];

    if v != 12 {
        std::process::exit(1);
    }
    if u != 5 {
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

    0
}