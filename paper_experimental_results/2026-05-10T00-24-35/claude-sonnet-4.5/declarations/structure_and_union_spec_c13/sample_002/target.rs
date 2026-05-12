use std::process::exit;

type Uptr = usize;

#[repr(C)]
struct Outer {
    base: i32,
    
    anonymous1: Anonymous1,
    
    anonymous2: Anonymous2,
}

#[repr(C)]
struct Anonymous1 {
    a: i32,
    
    anonymous_union1: AnonymousUnion1,
    
    d: i32,
}

#[repr(C)]
union AnonymousUnion1 {
    u_as_int: i32,
    
    anonymous_struct1: std::mem::ManuallyDrop<AnonymousStruct1>,
    
    u_as_unsigned: u32,
}

#[repr(C)]
struct AnonymousStruct1 {
    b: i32,
    
    anonymous_union2: AnonymousUnion2,
    
    c: i32,
}

#[repr(C)]
union AnonymousUnion2 {
    deep: i32,
    anonymous_struct2: std::mem::ManuallyDrop<AnonymousStruct2>,
}

#[repr(C)]
struct AnonymousStruct2 {
    x: i32,
    y: i32,
}

#[repr(C)]
union Anonymous2 {
    tail_i: i32,
    
    anonymous_struct3: std::mem::ManuallyDrop<AnonymousStruct3>,
}

#[repr(C)]
struct AnonymousStruct3 {
    tail_x: i32,
    
    anonymous_union3: AnonymousUnion3,
}

#[repr(C)]
union AnonymousUnion3 {
    tail_y: i32,
    anonymous_struct4: std::mem::ManuallyDrop<AnonymousStruct4>,
}

#[repr(C)]
struct AnonymousStruct4 {
    tail_p: i32,
    tail_q: i32,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        anonymous1: Anonymous1 {
            a: 1,
            anonymous_union1: AnonymousUnion1 {
                anonymous_struct1: std::mem::ManuallyDrop::new(AnonymousStruct1 {
                    b: 2,
                    anonymous_union2: AnonymousUnion2 { deep: 99 },
                    c: 3,
                }),
            },
            d: 4,
        },
        anonymous2: Anonymous2 {
            anonymous_struct3: std::mem::ManuallyDrop::new(AnonymousStruct3 {
                tail_x: 7,
                anonymous_union3: AnonymousUnion3 { tail_y: 8 },
            }),
        },
    };

    if o.base != 10 { return 1; }
    if o.anonymous1.a != 1 { return 2; }
    if unsafe { o.anonymous1.anonymous_union1.anonymous_struct1.b } != 2 { return 3; }
    if unsafe { o.anonymous1.anonymous_union1.anonymous_struct1.anonymous_union2.deep } != 99 { return 4; }
    if unsafe { o.anonymous1.anonymous_union1.anonymous_struct1.c } != 3 { return 5; }
    if o.anonymous1.d != 4 { return 6; }
    if unsafe { o.anonymous2.anonymous_struct3.tail_x } != 7 { return 7; }
    if unsafe { o.anonymous2.anonymous_struct3.anonymous_union3.tail_y } != 8 { return 8; }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        anonymous1: Anonymous1 {
            a: 0,
            anonymous_union1: AnonymousUnion1 {
                anonymous_struct1: std::mem::ManuallyDrop::new(AnonymousStruct1 {
                    b: 0,
                    anonymous_union2: AnonymousUnion2 { deep: 0 },
                    c: 0,
                }),
            },
            d: 0,
        },
        anonymous2: Anonymous2 {
            anonymous_struct3: std::mem::ManuallyDrop::new(AnonymousStruct3 {
                tail_x: 0,
                anonymous_union3: AnonymousUnion3 { tail_y: 0 },
            }),
        },
    };

    unsafe {
        o.anonymous1.anonymous_union1.anonymous_struct1.anonymous_union2.deep = 0x11223344;

        if o.anonymous1.anonymous_union1.anonymous_struct1.anonymous_union2.anonymous_struct2.x != 0x11223344 {
            return 20;
        }

        o.anonymous1.anonymous_union1.anonymous_struct1.anonymous_union2.anonymous_struct2.x = 5;
        o.anonymous1.anonymous_union1.anonymous_struct1.anonymous_union2.anonymous_struct2.y = 6;
        if o.anonymous1.anonymous_union1.anonymous_struct1.anonymous_union2.anonymous_struct2.x != 5 {
            return 21;
        }
        if o.anonymous1.anonymous_union1.anonymous_struct1.anonymous_union2.anonymous_struct2.y != 6 {
            return 22;
        }

        o.anonymous2.anonymous_struct3.anonymous_union3.anonymous_struct4.tail_p = 40;
        o.anonymous2.anonymous_struct3.anonymous_union3.anonymous_struct4.tail_q = 41;
        if o.anonymous2.anonymous_struct3.anonymous_union3.anonymous_struct4.tail_p != 40 {
            return 23;
        }
        if o.anonymous2.anonymous_struct3.anonymous_union3.anonymous_struct4.tail_q != 41 {
            return 24;
        }

        o.anonymous2.anonymous_struct3.anonymous_union3.tail_y = -9;
        if o.anonymous2.anonymous_struct3.anonymous_union3.tail_y != -9 {
            return 25;
        }
    }

    0
}

fn check_addressability() -> i32 {
    let mut o = Outer {
        base: 0,
        anonymous1: Anonymous1 {
            a: 0,
            anonymous_union1: AnonymousUnion1 {
                anonymous_struct1: std::mem::ManuallyDrop::new(AnonymousStruct1 {
                    b: 0,
                    anonymous_union2: AnonymousUnion2 { deep: 0 },
                    c: 0,
                }),
            },
            d: 0,
        },
        anonymous2: Anonymous2 {
            anonymous_struct3: std::mem::ManuallyDrop::new(AnonymousStruct3 {
                tail_x: 0,
                anonymous_union3: AnonymousUnion3 { tail_y: 0 },
            }),
        },
    };

    let pa = &o.anonymous1.a as *const i32 as Uptr;
    let pb = unsafe { &o.anonymous1.anonymous_union1.anonymous_struct1.b as *const i32 as Uptr };
    let pdeep = unsafe { &o.anonymous1.anonymous_union1.anonymous_struct1.anonymous_union2.deep as *const i32 as Uptr };
    let ptx = unsafe { &o.anonymous2.anonymous_struct3.tail_x as *const i32 as Uptr };
    let pty = unsafe { &o.anonymous2.anonymous_struct3.anonymous_union3.tail_y as *const i32 as Uptr };

    if pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0 {
        return 30;
    }
    if pa == pb {
        return 31;
    }
    if ptx == pty {
        return 32;
    }

    0
}

fn main() {
    let r = check_designated_init();
    if r != 0 {
        exit(r);
    }

    let r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        exit(r);
    }

    let r = check_addressability();
    if r != 0 {
        exit(r);
    }

    exit(0);
}