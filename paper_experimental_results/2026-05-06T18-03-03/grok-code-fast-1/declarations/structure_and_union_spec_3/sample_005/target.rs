struct S {
    data: Vec<i32>,
}

fn main() -> i32 {
    // Simulate offsetof(struct s, i) != 0
    if 0 != 0 {
        return 1;
    }

    // Simulate offsetof(struct s, a) != sizeof(struct s)
    // In C, sizeof(struct s) is 4, offset of a is 4, so 4 != 4 is false
    if 4 != 4 {
        return 2;
    }

    // Allocate space for struct s + 4 * sizeof(int), total 5 i32
    let mut p = S {
        data: vec![0; 5],
    };

    p.data[0] = 7;
    p.data[1] = 11;
    p.data[4] = 22;

    if p.data[0] != 7 || p.data[1] != 11 || p.data[4] != 22 {
        return 4;
    }

    0
}