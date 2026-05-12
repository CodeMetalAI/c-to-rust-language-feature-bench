fn main() {
    let x = 3;
    let f = |a: i32| a + x;
    let x = 2;

    let t = |a: i32| a;
    let g = f;
    let q = |x: i32| x;
    let p = || -> i32 { 0 };
    let r = |x: i32, y: i32| x * 10 + y;
    let str = |x: &str| x;

    let v = f(10);
    let u = f(4);
    let w = t(g(0));

    let i = [q(1), r(2, 3), r(4, 0), r(0, 5), r(0, 0)];
    let c = [["hello", ""], ["", ""]];

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

    if c[0][0] != "hello" {
        std::process::exit(8);
    }
    if c[1][0] != "" {
        std::process::exit(9);
    }
}