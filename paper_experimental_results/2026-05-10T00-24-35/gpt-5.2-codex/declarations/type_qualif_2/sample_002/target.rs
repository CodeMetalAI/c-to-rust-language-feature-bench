use std::process::exit;

#[derive(Copy, Clone)]
struct S {
    mem: i32,
}

const CS: S = S { mem: 1 };

type A = [[i32; 3]; 2];
const A_CONST: A = [[4, 5, 6], [7, 8, 9]];

fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    let mut ncs = CS;

    let pi = &mut ncs.mem;
    *pi += 10;

    let pci = &CS.mem;

    if use_int(ncs.mem) != 11 {
        exit(1);
    }

    if *pci != 1 {
        exit(2);
    }

    if A_CONST[0][0] != 4 {
        exit(3);
    }
    if A_CONST[0][1] != 5 {
        exit(4);
    }
    if A_CONST[0][2] != 6 {
        exit(5);
    }
    if A_CONST[1][0] != 7 {
        exit(6);
    }
    if A_CONST[1][1] != 8 {
        exit(7);
    }
    if A_CONST[1][2] != 9 {
        exit(8);
    }

    exit(0);
}