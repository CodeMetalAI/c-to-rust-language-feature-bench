struct S {
    mem: i32,
}

static CS: S = S { mem: 1 };
static mut NCS: S = S { mem: 0 };

type A = [[i32; 3]; 2];
static A_CONST: A = [[4, 5, 6], [7, 8, 9]];

fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    NCS = CS;

    let pi: &mut i32 = &mut NCS.mem;
    *pi += 10;

    let pci: &i32 = &CS.mem;

    if use_int(NCS.mem) != 11 {
        return;
    }

    if *pci != 1 {
        return;
    }

    if A_CONST[0][0] != 4 {
        return;
    }
    if A_CONST[0][1] != 5 {
        return;
    }
    if A_CONST[0][2] != 6 {
        return;
    }
    if A_CONST[1][0] != 7 {
        return;
    }
    if A_CONST[1][1] != 8 {
        return;
    }
    if A_CONST[1][2] != 9 {
        return;
    }
}