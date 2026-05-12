macro_rules! str {
    ($s:expr) => {
        concat!(stringify!($s))
    };
}

macro_rules! xstr {
    ($s:expr) => {
        str!($s)
    };
}

macro_rules! INCFILE {
    ($n:expr) => {
        concat!("vers", stringify!($n))
    };
}

macro_rules! glue {
    ($a:expr, $b:expr) => {
        concat!($a, stringify!($b))
    };
}

macro_rules! xglue {
    ($a:expr, $b:expr) => {
        glue!($a, $b)
    };
}

macro_rules! dbgfmt {
    ($s:expr, $t:expr) => {
        concat!("x", stringify!($s), "= %d, x", stringify!($t), "= %s")
    };
}

fn main() {
    let x1 = 7;
    let x2 = "hi";

    let fmt = dbgfmt!(1, 2);
    let inc = format!("{}.h", INCFILE!(2));
    let a = glue!(HIGHLOW, "");
    let b = xglue!(HIGHLOW, "");

    assert_eq!(x1, 7);
    assert_eq!(x2.chars().next().unwrap(), 'h');
    assert_eq!(x2.chars().nth(1).unwrap(), 'i');
    assert_eq!(x2.chars().last().unwrap(), '\0');

    assert_eq!(fmt.len(), 15);
    assert_eq!(fmt.chars().nth(0).unwrap(), 'x');
    assert_eq!(fmt.chars().nth(1).unwrap(), '1');
    assert_eq!(fmt.chars().nth(2).unwrap(), '=');
    assert_eq!(fmt.chars().nth(3).unwrap(), ' ');
    assert_eq!(fmt.chars().nth(4).unwrap(), '%');
    assert_eq!(fmt.chars().nth(5).unwrap(), 'd');
    assert_eq!(fmt.chars().nth(6).unwrap(), ',');
    assert_eq!(fmt.chars().nth(7).unwrap(), ' ');
    assert_eq!(fmt.chars().nth(8).unwrap(), 'x');
    assert_eq!(fmt.chars().nth(9).unwrap(), '2');
    assert_eq!(fmt.chars().nth(10).unwrap(), '=');
    assert_eq!(fmt.chars().nth(11).unwrap(), ' ');
    assert_eq!(fmt.chars().nth(12).unwrap(), '%');
    assert_eq!(fmt.chars().nth(13).unwrap(), 's');
    assert_eq!(fmt.chars().last().unwrap(), '\0');

    assert_eq!(inc.len(), 8);
    assert_eq!(inc.chars().nth(0).unwrap(), 'v');
    assert_eq!(inc.chars().nth(1).unwrap(), 'e');
    assert_eq!(inc.chars().nth(2).unwrap(), 'r');
    assert_eq!(inc.chars().nth(3).unwrap(), 's');
    assert_eq!(inc.chars().nth(4).unwrap(), '2');
    assert_eq!(inc.chars().nth(5).unwrap(), '.');
    assert_eq!(inc.chars().nth(6).unwrap(), 'h');
    assert_eq!(inc.chars().last().unwrap(), '\0');

    assert_eq!(a.len(), 6);
    assert_eq!(a.chars().nth(0).unwrap(), 'h');
    assert_eq!(a.chars().nth(1).unwrap(), 'e');
    assert_eq!(a.chars().nth(2).unwrap(), 'l');
    assert_eq!(a.chars().nth(3).unwrap(), 'l');
    assert_eq!(a.chars().nth(4).unwrap(), 'o');
    assert_eq!(a.chars().last().unwrap(), '\0');

    assert_eq!(b.len(), 13);
    assert_eq!(b.chars().nth(0).unwrap(), 'h');
    assert_eq!(b.chars().nth(1).unwrap(), 'e');
    assert_eq!(b.chars().nth(2).unwrap(), 'l');
    assert_eq!(b.chars().nth(3).unwrap(), 'l');
    assert_eq!(b.chars().nth(4).unwrap(), 'o');
    assert_eq!(b.chars().nth(5).unwrap(), ',');
    assert_eq!(b.chars().nth(6).unwrap(), ' ');
    assert_eq!(b.chars().nth(7).unwrap(), 'w');
    assert_eq!(b.chars().nth(8).unwrap(), 'o');
    assert_eq!(b.chars().nth(9).unwrap(), 'r');
    assert_eq!(b.chars().nth(10).unwrap(), 'l');
    assert_eq!(b.chars().nth(11).unwrap(), 'd');
    assert_eq!(b.chars().last().unwrap(), '\0');

    println!("All assertions passed.");
}