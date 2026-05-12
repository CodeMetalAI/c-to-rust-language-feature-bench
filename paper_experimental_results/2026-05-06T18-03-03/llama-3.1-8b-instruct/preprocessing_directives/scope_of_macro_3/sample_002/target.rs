macro_rules! x {
    () => { 3 };
}

macro_rules! F {
    ($a:expr) => { $a + x!() };
}

macro_rules! t {
    ($a:expr) => { $a };
}

macro_rules! g {
    () => { F };
}

macro_rules! q {
    () => { };
    ($x:expr) => { $x };
}

macro_rules! p {
    () => { i32 };
}

macro_rules! r {
    ($x:expr, $y:expr) => { concat!($x, $y) };
}

macro_rules! str {
    ($x:expr) => { stringify!($x) };
}

fn main() {
    let v = F!(10);
    let u = F!(3, 4);
    let w = t!(g!())(0);

    let i: [i32; 5] = [
        q!(1),
        r!(2, 3),
        r!(4, ),
        r!(, 5),
        r!(, ),
    ];

    let c: [[char; 6]; 2] = [
        [str!(hello), str!()],
        [str!(), str!()],
    ];

    assert_eq!(v, 12);
    assert_eq!(u, 6);
    assert_eq!(w, 2);

    assert_eq!(i[0], 1);
    assert_eq!(i[1], 23);
    assert_eq!(i[2], 4);
    assert_eq!(i[3], 5);

    assert_eq!(c[0][0], 'h');
    assert_eq!(c[0][1], 'e');
    assert_eq!(c[0][2], 'l');
    assert_eq!(c[0][3], 'l');
    assert_eq!(c[0][4], 'o');
    assert_eq!(c[0][5], '\0');

    assert_eq!(c[1][0], '\0');

    println!("Success!");
}