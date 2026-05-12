macro_rules! x {
    () => { 3 };
}

macro_rules! F {
    ($a:expr) => ($a + x!());
    ($($a:expr),*) => ($($a + x!()),*);
}

macro_rules! t {
    ( $a:expr ) => { $a };
}

macro_rules! g {
    () => { F };
}

macro_rules! q {
    () => { };
    ( $x:expr ) => { $x };
    ( $x:expr , $y:expr ) => { $x$y };
    ( $x:expr , $y:expr , $z:expr ) => { $x$y$z };
    ( $x:expr , ) => { $x };
    ( ) , $y:expr ) => { $y };
    ( ) ) => { };
}

macro_rules! p {
    () => { int };
}

macro_rules! r {
    ( $x:expr , $y:expr ) => { $x$y };
    ( $x:expr , ) => { $x };
    ( , $y:expr ) => { $y };
    ( ) ) => { };
}

macro_rules! str {
    ( $x:expr ) => { # $x };
}

fn main() {
    let v: i32 = F!(10);
    let u: i32 = F!(3, 4);
    let w: i32 = t!(g!)!(0);

    let i = [
        q!(1),
        q!(2, 3),
        q!(4,  ),
        q!(  , 5),
        q!(  ,  ),
    ];
    let c = [str!(hello).as_bytes().to_vec(), str!( ).as_bytes().to_vec()];

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

    if c[0][0] as char != 'h' {
        panic!("c[0][0] != 'h'");
    }
    if c[0][1] as char != 'e' {
        panic!("c[0][1] != 'e'");
    }
    if c[0][2] as char != 'l' {
        panic!("c[0][2] != 'l'");
    }
    if c[0][3] as char != 'l' {
        panic!("c[0][3] != 'l'");
    }
    if c[0][4] as char != 'o' {
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