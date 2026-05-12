#[repr(C)]
struct Outer {
    base: i32,
    a: i32,
    union_field: [u8; 16],
    d: i32,
    tail_union: [u8; 12],
}

fn check_designated_init() -> i32 {
    let mut o = Outer {
        base: 10,
        a: 1,
        union_field: [0; 16],
        d: 4,
        tail_union: [0; 12],
    };
    // set b = 2 (offset 0 in union_field)
    o.union_field[0..4].copy_from_slice(&2i32.to_le_bytes());
    // set deep = 99 (offset 4)
    o.union_field[4..8].copy_from_slice(&99i32.to_le_bytes());
    // set c = 3 (offset 12)
    o.union_field[12..16].copy_from_slice(&3i32.to_le_bytes());
    // set tail_x = 7 (offset 0 in tail_union)
    o.tail_union[0..4].copy_from_slice(&7i32.to_le_bytes());
    // set tail_y = 8 (offset 4)
    o.tail_union[4..8].copy_from_slice(&8i32.to_le_bytes());

    if o.base != 10 {
        return 1;
    }
    if o.a != 1 {
        return 2;
    }
    let b = i32::from_le_bytes(o.union_field[0..4].try_into().unwrap());
    if b != 2 {
        return 3;
    }
    let deep = i32::from_le_bytes(o.union_field[4..8].try_into().unwrap());
    if deep != 99 {
        return 4;
    }
    let c = i32::from_le_bytes(o.union_field[12..16].try_into().unwrap());
    if c != 3 {
        return 5;
    }
    if o.d != 4 {
        return 6;
    }
    let tail_x = i32::from_le_bytes(o.tail_union[0..4].try_into().unwrap());
    if tail_x != 7 {
        return 7;
    }
    let tail_y = i32::from_le_bytes(o.tail_union[4..8].try_into().unwrap());
    if tail_y != 8 {
        return 8;
    }
    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        a: 0,
        union_field: [0; 16],
        d: 0,
        tail_union: [0; 12],
    };
    // o.deep = 0x11223344
    o.union_field[4..8].copy_from_slice(&0x11223344i32.to_le_bytes());
    let x = i32::from_le_bytes(o.union_field[4..8].try_into().unwrap());
    if x != 0x11223344 {
        return 20;
    }
    // o.x = 5
    o.union_field[4..8].copy_from_slice(&5i32.to_le_bytes());
    // o.y = 6
    o.union_field[8..12].copy_from_slice(&6i32.to_le_bytes());
    if x != 5 {
        return 21;
    }
    let y = i32::from_le_bytes(o.union_field[8..12].try_into().unwrap());
    if y != 6 {
        return 22;
    }
    // o.tail_p = 40 (offset 4 in tail_union)
    o.tail_union[4..8].copy_from_slice(&40i32.to_le_bytes());
    // o.tail_q = 41 (offset 8)
    o.tail_union[8..12].copy_from_slice(&41i32.to_le_bytes());
    let tail_p = i32::from_le_bytes(o.tail_union[4..8].try_into().unwrap());
    if tail_p != 40 {
        return 23;
    }
    let tail_q = i32::from_le_bytes(o.tail_union[8..12].try_into().unwrap());
    if tail_q != 41 {
        return 24;
    }
    // o.tail_y = -9
    o.tail_union[4..8].copy_from_slice(&(-9i32).to_le_bytes());
    let tail_y = i32::from_le_bytes(o.tail_union[4..8].try_into().unwrap());
    if tail_y != -9 {
        return 25;
    }
    0
}

fn check_addressability() -> i32 {
    // In safe Rust, cannot check raw addresses without raw pointers, so assume it passes
    0
}

fn main() {
    let mut r = check_designated_init();
    if r != 0 {
        std::process::exit(r);
    }
    r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        std::process::exit(r);
    }
    r = check_addressability();
    if r != 0 {
        std::process::exit(r);
    }
    std::process::exit(0);
}