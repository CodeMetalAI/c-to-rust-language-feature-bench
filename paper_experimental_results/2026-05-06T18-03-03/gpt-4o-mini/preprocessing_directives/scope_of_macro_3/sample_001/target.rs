const X: i32 = 2;

fn f(a: i32) -> i32 {
    a + X
}

macro_rules! t {
    ($a:expr) => { $a };
}

macro_rules! g {
    ($a:expr) => { f($a) };
}

macro_rules! q {
    ($x:expr) => { $x };
}

macro_rules! r {
    ($x:ident, $y:ident) => { concat_idents!($x, $y) };
}

macro_rules! str {
    ($x:expr) => { $x };
}

fn main() -> i32 {
    let v = f(10);
    let u = f((3, 4));
    let w = t!(g!(0));

    let i: [i32; 4] = [q!(1), r!(2, 3), r!(4, ), r!(, 5)];
    let c: [[char; 6]; 2] = [[str!("hello")[0], str!("hello")[1], str!("hello")[2], str!("hello")[3], str!("hello")[4], '\0'], ['\0', '\0', '\0', '\0', '\0', '\0']];

    if v != 12 {
        return 1;
    }
    if u != 6 {
        return 2;
    }
    if w != 2 {
        return 3;
    }

    if i[0] != 1 {
        return 4;
    }
    if i[1] != 23 {
        return 5;
    }
    if i[2] != 4 {
        return 6;
    }
    if i[3] != 5 {
        return 7;
    }

    if c[0][0] != 'h' {
        return 8;
    }
    if c[0][1] != 'e' {
        return 9;
    }
    if c[0][2] != 'l' {
        return 10;
    }
    if c[0][3] != 'l' {
        return 11;
    }
    if c[0][4] != 'o' {
        return 12;
    }
    if c[0][5] != '\0' {
        return 13;
    }

    if c[1][0] != '\0' {
        return 14;
    }

    return 0;
}