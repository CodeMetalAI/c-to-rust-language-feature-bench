struct S {
    mem: i32,
}

static CS: S = S { mem: 1 };
static mut NCS: S = S { mem: 0 };

type A = [[i32; 3]; 2];
static A_ARRAY: A = [[4, 5, 6], [7, 8, 9]];

fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    NCS = CS;

    let pi: &mut i32 = &mut NCS.mem;
    *pi += 10;

    let pci: &i32 = &CS.mem;

    if use_int(NCS.mem) != 11 {
        return std::process::exit(1);
    }

    if *pci != 1 {
        return std::process::exit(2);
    }

    if A_ARRAY[0][0] != 4 {
        return std::process::exit(3);
    }
    if A_ARRAY[0][1] != 5 {
        return std::process::exit(4);
    }
    if A_ARRAY[0][2] != 6 {
        return std::process::exit(5);
    }
    if A_ARRAY[1][0] != 7 {
        return std::process::exit(6);
    }
    if A_ARRAY[1][1] != 8 {
        return std::process::exit(7);
    }
    if A_ARRAY[1][2] != 9 {
        return std::process::exit(8);
    }

    std::process::exit(0);
}