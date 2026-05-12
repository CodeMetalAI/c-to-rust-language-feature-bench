fn main() {
    const X: i32 = 3;
    macro_rules! F {
        ($a:expr) => {{
            ($a) + X
        }};
    }
    const X: i32 = 2;

    macro_rules! t {
        ($a:ident) => {{
            $a
        }};
    }
    macro_rules! g {
        () => {
            F
        };
    }
    macro_rules! q {
        () => {
            1
        };
    }
    macro_rules! p {
        () => {
            i32
        };
    }
    macro_rules! r {
        ($x:tt, $y:tt) => {
            concat!($x, $y)
        };
    }
    macro_rules! str {
        ($x:tt) => {
            stringify!($x)
        };
    }

    let v = F!(10);
    let u = F!((3, 4));
    let w = t!(g!)(0);

    p!() i[q!()] = [q!(), r!(2, 3).parse().unwrap(), r!(4, ).parse().unwrap(), r!(, 5).parse().unwrap(), r!(, ).parse().unwrap()];
    let c = [str!(hello).as_bytes(), str!().as_bytes()];

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

    if c[0][0] != b'h' {
        std::process::exit(8);
    }
    if c[0][1] != b'e' {
        std::process::exit(9);
    }
    if c[0][2] != b'l' {
        std::process::exit(10);
    }
    if c[0][3] != b'l' {
        std::process::exit(11);
    }
    if c[0][4] != b'o' {
        std::process::exit(12);
    }
    if c[0][5] != b'\0' {
        std::process::exit(13);
    }

    if c[1][0] != b'\0' {
        std::process::exit(14);
    }

    std::process::exit(0);
}