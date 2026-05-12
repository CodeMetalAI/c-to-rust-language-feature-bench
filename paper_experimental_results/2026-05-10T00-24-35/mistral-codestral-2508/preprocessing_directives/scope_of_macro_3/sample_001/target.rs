fn main() {
    // Define constants and functions
    const X: i32 = 3;
    fn f(a: i32) -> i32 { a + X }
    const X: i32 = 2;

    fn t<F>(a: F) -> F { a }
    fn g(a: i32) -> i32 { f(a) }
    fn q<T>(x: T) -> T { x }
    type P = i32;
    fn r(x: i32, y: i32) -> i32 { x * 10 + y }
    macro_rules! str {
        (hello) => { "hello" };
        () => { "" };
    }

    // Initialize variables
    let v = f(10);
    let u = f((3, 4).1);
    let w = t(g)(0);

    let i: [P; q(4)] = [q(1), r(2, 3), r(4, 0), r(0, 5), r(0, 0)];
    let c: [[char; 6]; 2] = [str!(hello).chars().collect::<Vec<_>>().try_into().unwrap(), str!().chars().collect::<Vec<_>>().try_into().unwrap()];

    // Check conditions
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

    std::process::exit(0);
}