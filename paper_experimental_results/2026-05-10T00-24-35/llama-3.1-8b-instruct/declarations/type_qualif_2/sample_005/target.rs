// type_qualif_2.rs
const STRUCT_S: struct {
    mem: i32,
} = STRUCT_S {
    mem: 1,
};

const UNION_A: [ [i32; 3 ]; 2 ] = [
    [4, 5, 6],
    [7, 8, 9],
];

fn use_int(x: i32) -> i32 {
    x
}

fn main() {
    let mut ncs = STRUCT_S {
        mem: 0,
    };
    ncs = STRUCT_S {
        mem: 1,
    };

    let mut pi = &mut ncs.mem;
    *pi += 10;

    let pci = &STRUCT_S {
        mem: 1,
    }.mem;

    if use_int(ncs.mem) != 11 {
        panic!("ncs.mem != 11");
    }

    if *pci != 1 {
        panic!("*pci != 1");
    }

    if UNION_A[0][0] != 4 {
        panic!("UNION_A[0][0] != 4");
    }
    if UNION_A[0][1] != 5 {
        panic!("UNION_A[0][1] != 5");
    }
    if UNION_A[0][2] != 6 {
        panic!("UNION_A[0][2] != 6");
    }
    if UNION_A[1][0] != 7 {
        panic!("UNION_A[1][0] != 7");
    }
    if UNION_A[1][1] != 8 {
        panic!("UNION_A[1][1] != 8");
    }
    if UNION_A[1][2] != 9 {
        panic!("UNION_A[1][2] != 9");
    }
}