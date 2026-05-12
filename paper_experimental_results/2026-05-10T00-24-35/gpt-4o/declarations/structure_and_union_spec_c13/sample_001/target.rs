struct InnerUnion {
    deep: i32,
}

struct InnerStruct {
    b: i32,
    nested_union: InnerUnion,
    c: i32,
}

struct OuterUnion {
    u_as_int: i32,
    u_as_unsigned: u32,
    nested_struct: InnerStruct,
}

struct TailUnion {
    tail_i: i32,
    nested_struct: TailStruct,
}

struct TailStruct {
    tail_x: i32,
    nested_union: TailInnerUnion,
}

struct TailInnerUnion {
    tail_y: i32,
    nested_struct: TailInnerStruct,
}

struct TailInnerStruct {
    tail_p: i32,
    tail_q: i32,
}

struct Outer {
    base: i32,
    a: i32,
    outer_union: OuterUnion,
    d: i32,
    tail_union: TailUnion,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        a: 1,
        outer_union: OuterUnion {
            u_as_int: 0,
            u_as_unsigned: 0,
            nested_struct: InnerStruct {
                b: 2,
                nested_union: InnerUnion { deep: 99 },
                c: 3,
            },
        },
        d: 4,
        tail_union: TailUnion {
            tail_i: 0,
            nested_struct: TailStruct {
                tail_x: 7,
                nested_union: TailInnerUnion {
                    tail_y: 8,
                    nested_struct: TailInnerStruct { tail_p: 0, tail_q: 0 },
                },
            },
        },
    };

    if o.base != 10 { return 1; }
    if o.a != 1 { return 2; }
    if o.outer_union.nested_struct.b != 2 { return 3; }
    if o.outer_union.nested_struct.nested_union.deep != 99 { return 4; }
    if o.outer_union.nested_struct.c != 3 { return 5; }
    if o.d != 4 { return 6; }
    if o.tail_union.nested_struct.tail_x != 7 { return 7; }
    if o.tail_union.nested_struct.nested_union.tail_y != 8 { return 8; }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        a: 0,
        outer_union: OuterUnion {
            u_as_int: 0,
            u_as_unsigned: 0,
            nested_struct: InnerStruct {
                b: 0,
                nested_union: InnerUnion { deep: 0 },
                c: 0,
            },
        },
        d: 0,
        tail_union: TailUnion {
            tail_i: 0,
            nested_struct: TailStruct {
                tail_x: 0,
                nested_union: TailInnerUnion {
                    tail_y: 0,
                    nested_struct: TailInnerStruct { tail_p: 0, tail_q: 0 },
                },
            },
        },
    };

    o.outer_union.nested_struct.nested_union.deep = 0x11223344;
    if o.outer_union.nested_struct.nested_union.deep != 0x11223344 { return 20; }

    o.outer_union.nested_struct.nested_union.deep = 5;
    o.outer_union.nested_struct.nested_union.deep = 6;
    if o.outer_union.nested_struct.nested_union.deep != 5 { return 21; }
    if o.outer_union.nested_struct.nested_union.deep != 6 { return 22; }

    o.tail_union.nested_struct.nested_union.nested_struct.tail_p = 40;
    o.tail_union.nested_struct.nested_union.nested_struct.tail_q = 41;
    if o.tail_union.nested_struct.nested_union.nested_struct.tail_p != 40 { return 23; }
    if o.tail_union.nested_struct.nested_union.nested_struct.tail_q != 41 { return 24; }

    o.tail_union.nested_struct.nested_union.tail_y = -9;
    if o.tail_union.nested_struct.nested_union.tail_y != -9 { return 25; }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        a: 0,
        outer_union: OuterUnion {
            u_as_int: 0,
            u_as_unsigned: 0,
            nested_struct: InnerStruct {
                b: 0,
                nested_union: InnerUnion { deep: 0 },
                c: 0,
            },
        },
        d: 0,
        tail_union: TailUnion {
            tail_i: 0,
            nested_struct: TailStruct {
                tail_x: 0,
                nested_union: TailInnerUnion {
                    tail_y: 0,
                    nested_struct: TailInnerStruct { tail_p: 0, tail_q: 0 },
                },
            },
        },
    };

    let pa = &o.a as *const _ as usize;
    let pb = &o.outer_union.nested_struct.b as *const _ as usize;
    let pdeep = &o.outer_union.nested_struct.nested_union.deep as *const _ as usize;
    let ptx = &o.tail_union.nested_struct.tail_x as *const _ as usize;
    let pty = &o.tail_union.nested_struct.nested_union.tail_y as *const _ as usize;

    if pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0 { return 30; }
    if pa == pb { return 31; }
    if ptx == pty { return 32; }

    0
}

fn main() -> i32 {
    let r = check_designated_init();
    if r != 0 {
        return r;
    }

    let r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        return r;
    }

    let r = check_addressability();
    if r != 0 {
        return r;
    }

    0
}