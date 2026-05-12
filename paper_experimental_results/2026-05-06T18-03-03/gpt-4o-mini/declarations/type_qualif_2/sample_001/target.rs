struct S {
    mem: i32,
}

const CS: S = S { mem: 1 };
static mut NCS: S = S { mem: 0 };

type A = [[i32; 3]; 2];
const A: A = [[4, 5, 6], [7, 8, 9]];

fn use_int(x: i32) -> i32 {
    x
}

fn main() -> i32 {
    let pi: &mut i32;
    let pci: &i32;

    unsafe {
        NCS = CS;
        pi = &mut NCS.mem;
        *pi += 10;

        pci = &CS.mem;
    }

    if use_int(NCS.mem) != 11 {
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