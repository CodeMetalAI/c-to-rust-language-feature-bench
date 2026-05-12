struct Pair {
    a: i32,
    b: i32,
}

fn main() {
    let p: [i32; 3] = [10, 20, 30];
    if p[0]!= 10 {
        return 1;
    }
    if p[1]!= 20 {
        return 2;
    }
    if p[2]!= 30 {
        return 3;
    }

    let q = Pair { a: 7, b: 11 };
    if q.a!= 7 {
        return 4;
    }
    if q.b!= 11 {
        return 5;
    }

    if [1, 2, 3][1]!= 2 {
        return 6;
    }

    std::process::exit(0);
}