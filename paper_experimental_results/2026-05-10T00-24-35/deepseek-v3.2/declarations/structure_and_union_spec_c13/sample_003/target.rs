#![allow(unused_parens)]

use std::mem;

type Uptr = usize;

struct DeepUnion {
    deep: i32,
    xy: (i32, i32),
}

impl DeepUnion {
    fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn deep(&self) -> i32 {
        self.deep
    }

    fn deep_mut(&mut self) -> &mut i32 {
        &mut self.deep
    }

    fn x(&self) -> i32 {
        self.xy.0
    }

    fn x_mut(&mut self) -> &mut i32 {
        &mut self.xy.0
    }

    fn y(&self) -> i32 {
        self.xy.1
    }

    fn y_mut(&mut self) -> &mut i32 {
        &mut self.xy.1
    }
}

struct InnerUnion {
    u_as_int: i32,
    nested: InnerNested,
    u_as_unsigned: u32,
}

impl InnerUnion {
    fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn u_as_int(&self) -> i32 {
        self.u_as_int
    }

    fn u_as_int_mut(&mut self) -> &mut i32 {
        &mut self.u_as_int
    }

    fn nested_mut(&mut self) -> &mut InnerNested {
        &mut self.nested
    }

    fn u_as_unsigned(&self) -> u32 {
        self.u_as_unsigned
    }

    fn u_as_unsigned_mut(&mut self) -> &mut u32 {
        &mut self.u_as_unsigned
    }
}

struct InnerNested {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

impl InnerNested {
    fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn b(&self) -> i32 {
        self.b
    }

    fn b_mut(&mut self) -> &mut i32 {
        &mut self.b
    }

    fn deep_union_mut(&mut self) -> &mut DeepUnion {
        &mut self.deep_union
    }

    fn c(&self) -> i32 {
        self.c
    }

    fn c_mut(&mut self) -> &mut i32 {
        &mut self.c
    }
}

struct InnerStruct {
    a: i32,
    inner_union: InnerUnion,
    d: i32,
}

impl InnerStruct {
    fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn a(&self) -> i32 {
        self.a
    }

    fn a_mut(&mut self) -> &mut i32 {
        &mut self.a
    }

    fn inner_union_mut(&mut self) -> &mut InnerUnion {
        &mut self.inner_union
    }

    fn d(&self) -> i32 {
        self.d
    }

    fn d_mut(&mut self) -> &mut i32 {
        &mut self.d
    }
}

struct TailUnion {
    tail_i: i32,
    nested_tail: TailStruct,
}

impl TailUnion {
    fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn tail_i(&self) -> i32 {
        self.tail_i
    }

    fn tail_i_mut(&mut self) -> &mut i32 {
        &mut self.tail_i
    }

    fn nested_tail_mut(&mut self) -> &mut TailStruct {
        &mut self.nested_tail
    }
}

struct TailInnerUnion {
    tail_y: i32,
    tail_pq: (i32, i32),
}

impl TailInnerUnion {
    fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn tail_y(&self) -> i32 {
        self.tail_y
    }

    fn tail_y_mut(&mut self) -> &mut i32 {
        &mut self.tail_y
    }

    fn tail_p(&self) -> i32 {
        self.tail_pq.0
    }

    fn tail_p_mut(&mut self) -> &mut i32 {
        &mut self.tail_pq.0
    }

    fn tail_q(&self) -> i32 {
        self.tail_pq.1
    }

    fn tail_q_mut(&mut self) -> &mut i32 {
        &mut self.tail_pq.1
    }
}

struct TailStruct {
    tail_x: i32,
    tail_inner_union: TailInnerUnion,
}

impl TailStruct {
    fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn tail_x(&self) -> i32 {
        self.tail_x
    }

    fn tail_x_mut(&mut self) -> &mut i32 {
        &mut self.tail_x
    }

    fn tail_inner_union_mut(&mut self) -> &mut TailInnerUnion {
        &mut self.tail_inner_union
    }
}

struct Outer {
    base: i32,
    inner_struct: InnerStruct,
    tail_union: TailUnion,
}

impl Outer {
    fn new() -> Self {
        unsafe { mem::zeroed() }
    }

    fn base(&self) -> i32 {
        self.base
    }

    fn base_mut(&mut self) -> &mut i32 {
        &mut self.base
    }

    fn inner_struct_mut(&mut self) -> &mut InnerStruct {
        &mut self.inner_struct
    }

    fn tail_union_mut(&mut self) -> &mut TailUnion {
        &mut self.tail_union
    }
}

fn check_designated_init() -> i32 {
    let mut o = Outer::new();
    *o.base_mut() = 10;
    *o.inner_struct_mut().a_mut() = 1;
    *o.inner_struct_mut().inner_union_mut().nested_mut().b_mut() = 2;
    *o.inner_struct_mut()
        .inner_union_mut()
        .nested_mut()
        .deep_union_mut()
        .deep_mut() = 99;
    *o.inner_struct_mut().inner_union_mut().nested_mut().c_mut() = 3;
    *o.inner_struct_mut().d_mut() = 4;
    *o.tail_union_mut()
        .nested_tail_mut()
        .tail_x_mut() = 7;
    *o.tail_union_mut()
        .nested_tail_mut()
        .tail_inner_union_mut()
        .tail_y_mut() = 8;

    if o.base() != 10 {
        return 1;
    }
    if o.inner_struct().a() != 1 {
        return 2;
    }
    if o.inner_struct().inner_union().nested().b() != 2 {
        return 3;
    }
    if o.inner_struct()
        .inner_union()
        .nested()
        .deep_union()
        .deep()
        != 99
    {
        return 4;
    }
    if o.inner_struct().inner_union().nested().c() != 3 {
        return 5;
    }
    if o.inner_struct().d() != 4 {
        return 6;
    }
    if o.tail_union().nested_tail().tail_x() != 7 {
        return 7;
    }
    if o.tail_union()
        .nested_tail()
        .tail_inner_union()
        .tail_y()
        != 8
    {
        return 8;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer::new();
    *o.base_mut() = 0;

    *o.inner_struct_mut()
        .inner_union_mut()
        .nested_mut()
        .deep_union_mut()
        .deep_mut() = 0x11223344;

    if o.inner_struct()
        .inner_union()
        .nested()
        .deep_union()
        .x()
        != 0x11223344
    {
        return 20;
    }

    *o.inner_struct_mut()
        .inner_union_mut()
        .nested_mut()
        .deep_union_mut()
        .x_mut() = 5;
    *o.inner_struct_mut()
        .inner_union_mut()
        .nested_mut()
        .deep_union_mut()
        .y_mut() = 6;
    if o.inner_struct()
        .inner_union()
        .nested()
        .deep_union()
        .x()
        != 5
    {
        return 21;
    }
    if o.inner_struct()
        .inner_union()
        .nested()
        .deep_union()
        .y()
        != 6
    {
        return 22;
    }

    *o.tail_union_mut()
        .nested_tail_mut()
        .tail_inner_union_mut()
        .tail_p_mut() = 40;
    *o.tail_union_mut()
        .nested_tail_mut()
        .tail_inner_union_mut()
        .tail_q_mut() = 41;
    if o.tail_union()
        .nested_tail()
        .tail_inner_union()
        .tail_p()
        != 40
    {
        return 23;
    }
    if o.tail_union()
        .nested_tail()
        .tail_inner_union()
        .tail_q()
        != 41
    {
        return 24;
    }

    *o.tail_union_mut()
        .nested_tail_mut()
        .tail_inner_union_mut()
        .tail_y_mut() = -9;
    if o.tail_union()
        .nested_tail()
        .tail_inner_union()
        .tail_y()
        != -9
    {
        return 25;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer::new();

    let pa = &o.inner_struct().a() as *const i32 as Uptr;
    let pb = &o.inner_struct().inner_union().nested().b() as *const i32 as Uptr;
    let pdeep = &o.inner_struct()
        .inner_union()
        .nested()
        .deep_union()
        .deep() as *const i32 as Uptr;
    let ptx = &o.tail_union().nested_tail().tail_x() as *const i32 as Uptr;
    let pty = &o.tail_union()
        .nested_tail()
        .tail_inner_union()
        .tail_y() as *const i32 as Uptr;

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