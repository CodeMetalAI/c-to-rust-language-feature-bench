fn main() {
    let x: i32 = 3;
    let f = |a: i32| -> i32 { a + x };
    let x: i32 = 2;

    let g = f;
    let u: i32 = f(3);
    let v: i32 = f(10);
    let w: i32 = g(0);

    let i: [i32; 5] = [1, 2 + 3, 4, 5, 0];
    let c = [
        ["h", "e", "l", "l", "o", "\0"],
        ["\0", "\0", "\0", "\0", "\0", "\0"],
    ];

    if v!= 13 {
        panic!("v!= 13");
    }
    if u!= 6 {
        panic!("u!= 6");
    }
    if w!= 2 {
        panic!("w!= 2");
    }

    if i[0]!= 1 {
        panic!("i[0]!= 1");
    }
    if i[1]!= 5 {
        panic!("i[1]!= 5");
    }
    if i[2]!= 4 {
        panic!("i[2]!= 4");
    }
    if i[3]!= 5 {
        panic!("i[3]!= 5");
    }
    if i[4]!= 0 {
        panic!("i[4]!= 0");
    }

    if c[0][0]!= 'h' {
        panic!("c[0][0]!= 'h'");
    }
    if c[0][1]!= 'e' {
        panic!("c[0][1]!= 'e'");
    }
    if c[0][2]!= 'l' {
        panic!("c[0][2]!= 'l'");
    }
    if c[0][3]!= 'l' {
        panic!("c[0][3]!= 'l'");
    }
    if c[0][4]!= 'o' {
        panic!("c[0][4]!= 'o'");
    }
    if c[0][5]!= '\0' {
        panic!("c[0][5]!= '\0'");
    }

    if c[1][0]!= '\0' {
        panic!("c[1][0]!= '\0'");
    }

    panic!("All tests passed");
}