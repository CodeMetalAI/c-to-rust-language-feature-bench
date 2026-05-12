#[derive(Debug, Copy, Clone)]
struct S {
    mem: i32,
}

const A: [[i32; 3]; 2] = [[4, 5, 6], [7, 8, 9]];

fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    const cs: S = S { mem: 1 };
    let mut ncs: S = S { mem: 0 };

    ncs = cs;

    let pi = &mut ncs.mem;
    *pi += 10;

    let pci = &cs.mem;

    if use_int(ncs.mem) != 11 {
        return 1;
    }

    if *pci != 1 {
        return 2;
    }

    if A[0][0] != 4 {
        return 3;
    }
    if A[0][1] != 5 {
        return 4;
    }
    if A[0][2] != 6 {
        return 5;
    }
    if A[1][0] != 7 {
        return 6;
    }
    if A[1][1] != 8 {
        return 7;
    }
    if A[1][2] != 9 {
        return 8;
    }

    0
}