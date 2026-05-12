fn main() {
    let a: i32;
    let b: i32;
    let c: i32;
    let d: i32;

    a = 1;
    b = 2;
    c = 3;
    d = 4;
    if ((a < b) == (c < d)) != true {
        std::process::exit(1);
    }

    let a = 2;
    let b = 1;
    let c = 4;
    let d = 3;
    if ((a < b) == (c < d)) != true {
        std::process::exit(2);
    }

    let a = 1;
    let b = 2;
    let c = 4;
    let d = 3;
    if ((a < b) == (c < d)) != false {
        std::process::exit(3);
    }

    let a = 2;
    let b = 1;
    let c = 3;
    let d = 4;
    if ((a < b) == (c < d)) != false {
        std::process::exit(4);
    }

    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;
    if ((a < b) == (c < d)) != true {
        std::process::exit(5);
    }

    std::process::exit(0);
}