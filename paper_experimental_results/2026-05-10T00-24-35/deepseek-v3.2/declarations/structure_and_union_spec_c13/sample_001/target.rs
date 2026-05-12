use std::mem;

type Uptr = usize;

#[repr(C)]
struct InnerUnionDeep {
    deep: i32,
    nested: InnerUnionDeepNested,
}

#[repr(C)]
union InnerUnionDeepUnion {
    deep: i32,
    nested: InnerUnionDeepNested,
}

#[repr(C)]
struct InnerUnionDeepStruct {
    b: i32,
    deep_union: InnerUnionDeepUnion,
    c: i32,
}

#[repr(C)]
union InnerUnion {
    u_as_int: i32,
    deep_struct: InnerUnionDeepStruct,
    u_as_unsigned: u32,
}

#[repr(C)]
struct InnerStruct {
    a: i32,
    inner_union: InnerUnion,
    d: i32,
}

#[repr(C)]
struct TailUnionNested {
    tail_y: i32,
    tail_struct: TailUnionNestedStruct,
}

#[repr(C)]
union TailUnionUnion {
    tail_y: i32,
    nested: TailUnionNested,
}

#[repr(C)]
struct TailUnionStruct {
    tail_x: i32,
    tail_union: TailUnionUnion,
}

#[repr(C)]
union TailUnion {
    tail_i: i32,
    tail_struct: TailUnionStruct,
}

#[repr(C)]
struct Outer {
    base: i32,
    inner: InnerStruct,
    tail: TailUnion,
}

fn check_designated_init() -> i32 {
    let mut deep_struct = InnerUnionDeepStruct {
        b: 2,
        deep_union: InnerUnionDeepUnion { deep: 99 },
        c: 3,
    };
    
    let mut inner_union = InnerUnion {
        deep_struct,
    };
    
    let inner = InnerStruct {
        a: 1,
        inner_union,
        d: 4,
    };
    
    let mut tail_nested = TailUnionNested {
        tail_y: 8,
        tail_struct: TailUnionNestedStruct { tail_p: 0, tail_q: 0 },
    };
    
    let mut tail_union = TailUnionUnion {
        nested: tail_nested,
    };
    
    let tail_struct = TailUnionStruct {
        tail_x: 7,
        tail_union,
    };
    
    let mut tail = TailUnion {
        tail_struct,
    };
    
    let o = Outer {
        base: 10,
        inner,
        tail,
    };
    
    if o.base != 10 {
        return 1;
    }
    if o.inner.a != 1 {
        return 2;
    }
    
    unsafe {
        if o.inner.inner_union.deep_struct.b != 2 {
            return 3;
        }
        if o.inner.inner_union.deep_struct.deep_union.deep != 99 {
            return 4;
        }
        if o.inner.inner_union.deep_struct.c != 3 {
            return 5;
        }
    }
    
    if o.inner.d != 4 {
        return 6;
    }
    
    unsafe {
        if o.tail.tail_struct.tail_x != 7 {
            return 7;
        }
        if o.tail.tail_struct.tail_union.nested.tail_y != 8 {
            return 8;
        }
    }
    
    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: InnerStruct {
            a: 0,
            inner_union: InnerUnion { u_as_int: 0 },
            d: 0,
        },
        tail: TailUnion { tail_i: 0 },
    };
    
    unsafe {
        o.inner.inner_union.deep_struct.deep_union.deep = 0x11223344;
        
        if o.inner.inner_union.deep_struct.deep_union.nested.x != 0x11223344 {
            return 20;
        }
        
        o.inner.inner_union.deep_struct.deep_union.nested.x = 5;
        o.inner.inner_union.deep_struct.deep_union.nested.y = 6;
        
        if o.inner.inner_union.deep_struct.deep_union.nested.x != 5 {
            return 21;
        }
        if o.inner.inner_union.deep_struct.deep_union.nested.y != 6 {
            return 22;
        }
        
        o.tail.tail_struct.tail_union.nested.tail_struct.tail_p = 40;
        o.tail.tail_struct.tail_union.nested.tail_struct.tail_q = 41;
        
        if o.tail.tail_struct.tail_union.nested.tail_struct.tail_p != 40 {
            return 23;
        }
        if o.tail.tail_struct.tail_union.nested.tail_struct.tail_q != 41 {
            return 24;
        }
        
        o.tail.tail_struct.tail_union.tail_y = -9;
        
        if o.tail.tail_struct.tail_union.tail_y != -9 {
            return 25;
        }
    }
    
    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        inner: InnerStruct {
            a: 0,
            inner_union: InnerUnion { u_as_int: 0 },
            d: 0,
        },
        tail: TailUnion { tail_i: 0 },
    };
    
    let pa = &o.inner.a as *const i32 as Uptr;
    let pb = unsafe { &o.inner.inner_union.deep_struct.b as *const i32 as Uptr };
    let pdeep = unsafe { &o.inner.inner_union.deep_struct.deep_union.deep as *const i32 as Uptr };
    let ptx = unsafe { &o.tail.tail_struct.tail_x as *const i32 as Uptr };
    let pty = unsafe { &o.tail.tail_struct.tail_union.tail_y as *const i32 as Uptr };
    
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
        std::process::exit(r);
    }
    
    let r = check_union_aliasing_via_flattened_names();
    if r != 0 {
        std::process::exit(r);
    }
    
    let r = check_addressability();
    if r != 0 {
        std::process::exit(r);
    }
    
    std::process::exit(0);
}

#[repr(C)]
struct InnerUnionDeepNested {
    x: i32,
    y: i32,
}

#[repr(C)]
struct TailUnionNestedStruct {
    tail_p: i32,
    tail_q: i32,
}