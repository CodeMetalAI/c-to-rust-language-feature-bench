fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    struct S {
        mem: i32,
    }

    const CS: S = S { mem: 1 };
    let mut ncs = S { mem: 0 };

    type A = [[i32; 3]; 2];
    const A_CONST: A = [[4, 5, 6], [7, 8, 9]];

    ncs = CS;

    let mut pi: *mut i32 = &mut ncs.mem;
    unsafe {
        *pi += 10;
    }

    let pci: *const i32 = &CS.mem;

    if use_int(ncs.mem) != 11 {
        std::process::exit(1);
    }

    if unsafe { *pci } != 1 {
        std::process::exit(2);
    }

    if A_CONST[0][0] != 4 {
        std::process::exit(3);
    }
    if A_CONST[0][1] != 5 {
        std::process::exit(4);
    }
    if A_CONST[0][2] != 6 {
        std::process::exit(5);
    }
    if A_CONST[1][0] != 7 {
        std::process::exit(6);
    }
    if A_CONST[1][1] != 8 {
        std::process::exit(7);
    }
    if A_CONST[1][2] != 9 {
        std::process::exit(8);
    }
}