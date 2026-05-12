fn main() {
    const X: i32 = 2;

    fn f(a: i32) -> i32 {
        a + X
    }

    fn r(x: i32, y: i32) -> i32 {
        // This function is equivalent to the token pasting in macros.
        if x == 0 {
            y
        } else if y == 0 {
            x
        } else {
            x * 10 + y
        }
    }

    fn str_lit(x: &str) -> &str {
        x
    }

    let v = f(10);
    let u = f(4); // (3, 4) evaluates to 4 in C due to comma operator
    let w = f(0);

    let i = [1, r(2, 3), 4, r(0, 5)];

    let c: [&[u8; 6]; 2] = [
        str_lit("hello").as_bytes_array_6(),
        str_lit("").as_bytes_array_6(),
    ];

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
}

trait AsBytesArray6 {
    fn as_bytes_array_6(&self) -> &[u8; 6];
}

impl AsBytesArray6 for &str {
    fn as_bytes_array_6(&self) -> &[u8; 6] {
        let mut array = [0; 6];
        let bytes = self.as_bytes();
        for (i, &b) in bytes.iter().enumerate() {
            if i < 6 {
                array[i] = b;
            }
        }
        &array
    }
}