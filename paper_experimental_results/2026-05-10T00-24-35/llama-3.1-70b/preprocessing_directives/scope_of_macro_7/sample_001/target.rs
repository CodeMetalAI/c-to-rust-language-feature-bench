fn main() {
    let x = 7;
    let y = 3;

    if count_args("Flag")!= 1 {
        std::process::exit(1);
    }
    if count_args("X = %d\n", x)!= 2 {
        std::process::exit(2);
    }

    if showlist("The first, second, and third items.")!= "The first, second, and third items." {
        std::process::exit(3);
    }

    if report(x > y, "x is %d but y is %d", x, y)!= 1 {
        std::process::exit(4);
    }
    if report(x < y, "x is %d but y is %d", x, y)!= 3 {
        std::process::exit(5);
    }

    std::process::exit(0);
}

fn count_args<T, U>(_t: T, _u: U) -> usize
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    2
}

fn count_args<T>(t: T) -> usize
where
    T: std::fmt::Display,
{
    1
}

fn count_args<T, U, V, W, X, Y, Z, A, B, C>(_t: T, _u: U, _v: V, _w: W, _x: X, _y: Y, _z: Z, _a: A, _b: B, _c: C) -> usize
where
    T: std::fmt::Display,
    U: std::fmt::Display,
    V: std::fmt::Display,
    W: std::fmt::Display,
    X: std::fmt::Display,
    Y: std::fmt::Display,
    Z: std::fmt::Display,
    A: std::fmt::Display,
    B: std::fmt::Display,
    C: std::fmt::Display,
{
    10
}

fn showlist<T: std::fmt::Display>(s: T) -> String {
    format!("{}", s)
}

fn report<T: std::fmt::Display, U: std::fmt::Display, V, W, X, Y, Z, A, B, C>(test: bool, s: T, _v: V, _w: W, _x: X, _y: Y, _z: Z, _a: A, _b: B, _c: C) -> usize
where
    V: std::fmt::Display,
    W: std::fmt::Display,
    X: std::fmt::Display,
    Y: std::fmt::Display,
    Z: std::fmt::Display,
    A: std::fmt::Display,
    B: std::fmt::Display,
    C: std::fmt::Display,
{
    if test {
        1
    } else {
        count_args(s, _v, _w, _x, _y, _z, _a, _b, _c)
    }
}