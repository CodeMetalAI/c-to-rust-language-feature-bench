// scope_of_macro_3.rs

macro_rules! x {
    () => { 3 };
}

macro_rules! F {
    ($a:expr) => { $a + x!() };
    ($($tt:tt)*) => { F!($($tt)*) };
}

macro_rules! t {
    ($a:expr) => { $a };
}

macro_rules! g {
    () => { F };
}

macro_rules! q {
    () => { () };
    ($x:expr) => { $x };
}

macro_rules! p {
    () => { int };
}

macro_rules! r {
    ($x:expr, $y:expr) => { $x $y };
}

macro_rules! str {
    ($x:expr) => { stringify!($x) };
}

fn main() {
    let v = F!(10);
    let u = F!((3, 4));
    let w = t!(g!())(0);

    let i: [i32; 5] = [
        q!(1),
        r!(2, 3),
        r!(4, ),
        r!(, 5),
        r!(, ),
    ];

    let c: [[char; 6]; 2] = [
        [str!(hello).as_bytes()[0], str!(hello).as_bytes()[1], str!(hello).as_bytes()[2], str!(hello).as_bytes()[3], str!(hello).as_bytes()[4], str!(hello).as_bytes()[5]],
        [str!().as_bytes()[0], str!().as_bytes()[1], str!().as_bytes()[2], str!().as_bytes()[3], str!().as_bytes()[4], str!().as_bytes()[5]],
    ];

    if v != 12 {
        panic!("v != 12");
    }
    if u != 6 {
        panic!("u != 6");
    }
    if w != 2 {
        panic!("w != 2");
    }

    if i[0] != 1 {
        panic!("i[0] != 1");
    }
    if i[1] != 23 {
        panic!("i[1] != 23");
    }
    if i[2] != 4 {
        panic!("i[2] != 4");
    }
    if i[3] != 5 {
        panic!("i[3] != 5");
    }

    if c[0][0] != 'h' {
        panic!("c[0][0] != 'h'");
    }
    if c[0][1] != 'e' {
        panic!("c[0][1] != 'e'");
    }
    if c[0][2] != 'l' {
        panic!("c[0][2] != 'l'");
    }
    if c[0][3] != 'l' {
        panic!("c[0][3] != 'l'");
    }
    if c[0][4] != 'o' {
        panic!("c[0][4] != 'o'");
    }
    if c[0][5] != '\0' {
        panic!("c[0][5] != '\0'");
    }

    if c[1][0] != '\0' {
        panic!("c[1][0] != '\0'");
    }

    println!("All tests passed!");
}