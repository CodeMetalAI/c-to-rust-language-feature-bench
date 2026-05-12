fn main() {
    // Macros are tricky to translate directly. We'll try to replicate the behavior.
    let x = 3;
    let v = F(10);
    let x = 2;
    let u = F((3, 4));
    let w = t(g)(0);

    let i = [1, 23, 4, 5];
    let c = ["hello", ""];

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

    if c[0][0] != 'h' {
        std::process::exit(8);
    }
    if c[0][1] != 'e' {
        std::process::exit(9);
    }
    if c[0][2] != 'l' {
        std::process::exit(10);
    }
    if c[0][3] != 'l' {
        std::process::exit(11);
    }
    if c[0][4] != 'o' {
        std::process::exit(12);
    }
    if c[0][5] != '\0' {
        std::process::exit(13);
    }

    if c[1][0] != '\0' {
        std::process::exit(14);
    }

    std::process::exit(0);
}

fn F(a: (i32, i32)) -> i32 {
    a.0 + 2
}

fn t<T>(f: fn(T) -> i32) -> fn(T) -> i32 {
    f
}

fn g(a: i32) -> i32 {
    F(a)
}