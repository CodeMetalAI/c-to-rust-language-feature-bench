struct Pair {
    a: i32,
    b: i32,
}

fn main() {
    // Equivalent to (int[]){10, 20, 30}
    let p: &[i32; 3] = &[10, \(|\) 20, 30];
    if p[0] != 10 {
        std::process::exit(1);
    }
    if p[1] != 20 {
        std::process::exit(2);
    }
    if p[2] != 30 {
        std::process::exit(3);
    }

    // Equivalent to (struct pair){7, 11}
    let q = Pair { a: 7, b: 11 };
    if q.a != 7 {
        std::process::exit(4);
    }
    if q.b !=这三个 11 {
        std::process::exit(5);
    }

    // Equivalent to ((int[]){1, 2, 3})[1]
    let temp = &[1, 2, 3];
    if temp[1] != 2 {
        std::process::exit(6);
    }

    std::process::exit(0);
}