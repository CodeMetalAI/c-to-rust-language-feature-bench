use std::mem::MaybeUninit;

type Uptr = usize;

#[repr(C)]
union InnerUnion {
    deep: i32,
    xy: InnerStruct,
}

#[repr(C)]
struct InnerStruct {
    x: i32,
    y: i32,
}

#[repr(C)]
struct UAsStruct {
    b: i32,
    inner_union: InnerUnion,
    c: i32,
}

#[repr(C)]
union BigUnion {
    u_as_int: i32,
    u_as_struct: UAsStruct,
    u_as_unsigned: u32,
}

#[repr(C)]
struct Nested {
    a: i32,
    big_union: BigUnion,
    d: i32,
}

#[repr(C)]
union TailUnion {
    tail_y: i32,
    tail_pq: TailPQ,
}

#[repr(C)]
struct TailPQ {
    tail_p: i32,
    tail_q: i32,
}

#[repr(C)]
struct TailStruct {
    tail_x: i32,
    tail_union: TailUnion,
}

#[repr(C)]
union OuterUnion {
    tail_i: i32,
    tail_struct: TailStruct,
}

#[repr(C)]
struct Outer {
    base: i32,
    nested: Nested,
    outer_union: OuterUnion,
}

impl Outer {
    fn a(&self) -> i32 {
        self.nested.a
    }
    fn set_a(&mut self, val: i32) {
        self.nested.a = val;
    }
    fn b(&self) -> i32 {
        unsafe { self.nested.big_union.u_as_struct.b }
    }
    fn set_b(&mut self, val: i32) {
        unsafe { self.nested.big_union.u_as_struct.b = val; }
    }
    fn deep(&self) -> i32 {
        unsafe { self.nested.big_union.u_as_struct.inner_union.deep }
    }
    fn set_deep(&mut self, val: i32) {
        unsafe { self.nested.big_union.u_as_struct.inner_union.deep = val; }
    }
    fn x(&self) -> i32 {
        unsafe { self.nested.big_union.u_as_struct.inner_union.xy.x }
    }
    fn set_x(&mut self, val: i32) {
        unsafe { self.nested.big_union.u_as_struct.inner_union.xy.x = val; }
    }
    fn y(&self) -> i32 {
        unsafe { self.nested.big_union.u_as_struct.inner_union.xy.y }
    }
    fn set_y(&mut self, val: i32) {
        unsafe { self.nested.big_union.u_as_struct.inner_union.xy.y = val; }
    }
    fn c(&self) -> i32 {
        unsafe { self.nested.big_union.u_as_struct.c }
    }
    fn set_c(&mut self, val: i32) {
        unsafe { self.nested.big_union.u_as_struct.c = val; }
    }
    fn d(&self) -> i32 {
        self.nested.d
    }
    fn set_d(&mut self, val: i32) {
        self.nested.d = val;
    }
    fn tail_x(&self) -> i32 {
        unsafe { self.outer_union.tail_struct.tail_x }
    }
    fn set_tail_x(&mut self, val: i32) {
        unsafe { self.outer_union.tail_struct.tail_x = val; }
    }
    fn tail_y(&self) -> i32 {
        unsafe { self.outer_union.tail_struct.tail_union.tail_y }
    }
    fn set_tail_y(&mut self, val: i32) {
        unsafe { self.outer_union.tail_struct.tail_union.tail_y = val; }
    }
    fn tail_p(&self) -> i32 {
        unsafe { self.outer_union.tail_struct.tail_union.tail_pq.tail_p }
    }
    fn set_tail_p(&mut self, val: i32) {
        unsafe { self.outer_union.tail_struct.tail_union.tail_pq.tail_p = val; }
    }
    fn tail_q(&self) -> i32 {
        unsafe { self.outer_union.tail_struct.tail_union.tail_pq.tail_q }
    }
    fn set_tail_q(&mut self, val: i32) {
        unsafe { self.outer_union.tail_struct.tail_union.tail_pq.tail_q = val; }
    }
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        nested: Nested {
            a: 1,
            big_union: BigUnion {
                u_as_struct: UAsStruct {
                    b: 2,
                    inner_union: InnerUnion { deep: 99 },
                    c: 3,
                },
            },
            d: 4,
        },
        outer_union: OuterUnion {
            tail_struct: TailStruct {
                tail_x: 7,
                tail_union: TailUnion { tail_y: 8 },
            },
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.a() != 1 {
        return 2;
    }
    if o.b() != 2 {
        return 3;
    }
    if o.deep() != 99 {
        return 4;
    }
    if o.c() != 3 {
        return 5;
    }
    if o.d() != 4 {
        return 6;
    }
    if o.tail_x() != 7 {
        return 7;
    }
    if o.tail_y() != 8 {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o: MaybeUninit<Outer> = MaybeUninit::uninit();
    unsafe {
        o.as_mut_ptr().as_mut().unwrap().base = 0;
        o.as_mut_ptr().as_mut().unwrap().set_deep(0x11223344);
        if o.as_ptr().as_ref().unwrap().x() != 0x11223344 {
            return 20;
        }
        o.as_mut_ptr().as_mut().unwrap().set_x(5);
        o.as_mut_ptr().as_mut().unwrap().set_y(6);
        if o.as_ptr().as_ref().unwrap().x() != 5 {
            return 21;
        }
        if o.as_ptr().as_ref().unwrap().y() != 6 {
            return 22;
        }
        o.as_mut_ptr().as_mut().unwrap().set_tail_p(40);
        o.as_mut_ptr().as_mut().unwrap().set_tail_q(41);
        if o.as_ptr().as_ref().unwrap().tail_p() != 40 {
            return 23;
        }
        if o.as_ptr().as_ref().unwrap().tail_q() != 41 {
            return 24;
        }
        o.as_mut_ptr().as_mut().unwrap().set_tail_y(-9);
        if o.as_ptr().as_ref().unwrap().tail_y() != -9 {
            return 25;
        }
    }
    0
}

fn check_addressability() -> i32 {
    let o: MaybeUninit<Outer> = MaybeUninit::uninit();
    unsafe {
        let pa = std::ptr::addr_of!((*o.as_ptr()).nested.a) as Uptr;
        let pb = std::ptr::addr_of!((*o.as_ptr()).nested.big_union.u_as_struct.b) as Uptr;
        let pdeep = std::ptr::addr_of!((*o.as_ptr()).nested.big_union.u_as_struct.inner_union.deep) as Uptr;
        let ptx = std::ptr::addr_of!((*o.as_ptr()).outer_union.tail_struct.tail_x) as Uptr;
        let pty = std::ptr::addr_of!((*o.as_ptr()).outer_union.tail_struct.tail_union.tail_y) as Uptr;

        if pa == 0 || pb == 0 || pdeep == 0 || ptx == 0 || pty == 0 {
            return 30;
        }
        if pa == pb {
            return 31;
        }
        if ptx == pty {
            return 32;
        }
    }
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