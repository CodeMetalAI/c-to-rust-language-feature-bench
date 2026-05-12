macro_rules! x {
    () => {
        3
    };
}

macro_rules! F {
    ( $a:expr ) => {
        $a + x!()
    };
}

macro_rules! t {
    ( $a:expr ) => {
        $a
    };
}

macro_rules! g {
    () => {
        F
    };
}

macro_rules! q {
    () => {
        String::new()
    };
}

macro_rules! p {
    () => {
        String
    };
}

macro_rules! r {
    ( $x:expr, $y:expr ) => {
        concat!(stringify!($x), stringify!($y))
    };
}

macro_rules! str {
    ( $x:expr ) => {
        stringify!($x)
    };
}

fn main() {
    let v = F!(10);
    let u = F!((3, 4));
    let w = t!(g!())(0);

    let i: Vec<String> = vec![
        q!(),
        String::from(r!(2, 3)),
        String::from(r!(4,)),
        String::from(r!(, 5)),
        String::from(r!(,)),
    ];
    let c: Vec<String> = vec![
        str!(hello).to_string(),
        String::new(),
    ];

    assert_eq!(v, 12);
    assert_eq!(u, 6);
    assert_eq!(w, 2);

    assert_eq!(i[0], String::new());
    assert_eq!(i[1], "23".to_string());
    assert_eq!(i[2], "4".to_string());
    assert_eq!(i[3], "5".to_string());

    assert_eq!(c[0].as_bytes()[0], b'h');
    assert_eq!(c[0].as_bytes()[1], b'e');
    assert_eq!(c[0].as_bytes()[2], b'l');
    assert_eq!(c[0].as_bytes()[3], b'l');
    assert_eq!(c[0].as_bytes()[4], b'o');
    assert_eq!(c[0].as_bytes()[5], b'\0');

    assert_eq!(c[1].as_bytes()[0], b'\0');

    println!("All assertions passed");
}