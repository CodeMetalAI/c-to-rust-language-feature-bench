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

type Uptr = usize;

struct DeepUnion {
    deep: i32,
    xy: Option<(i32, i32)>,
}

struct InnerUnion {
    u_as_int: i32,
    nested: Option<InnerNested>,
    u_as_unsigned: u32,
}

struct InnerNested {
    b: i32,
    deep_union: DeepUnion,
    c: i32,
}

struct InnerStruct {
    a: i32,
    union_field: InnerUnion,
    d: i32,
}

struct TailUnion {
    tail_i: i32,
    nested: Option<TailNested>,
}

struct TailNested {
    tail_x: i32,
    tail_union: TailDeepUnion,
}

struct TailDeepUnion {
    tail_y: i32,
    tail_pq: Option<(i32, i32)>,
}

struct Outer {
    base: i32,
    inner: InnerStruct,
    tail: TailUnion,
}

fn check_designated_init() -> i32 {
    let o = Outer {
        base: 10,
        inner: InnerStruct {
            a:1581,
            union_field: InnerUnion {
                u_as_int: 0,
                nested: Some(InnerNested {
                    b: 2,
                    deep_union: DeepUnion {
                        deep: 99,
                        xy: None,
                    },
                    c: 3,
                }),
                u_as_unsigned: 0,
            },
            d: 4,
        },
        tail: TailUnion {
            tail_i: 0,
            nested: Some(TailNested {
                tail_x: 7,
                tail_union: TailDeepUnion {
                    tail_y: 8,
                    tail_pq: None,
                },
            }),
        },
    };

    if o.base != 10 {
        return 1;
    }
    if o.inner.a != 1 {
        return 2;
    }
    if let Some(ref nested) = o.inner.union_field.nested {
        if nested.b != 2 {
            return 3;
        }
        if nested.deep_union.deep != 99 {
            return 4;
        }
        if nested.c != 3 {
            return 5;
        }
    } else {
        return 3;
    }
    if o.inner.d != 4 {
        return 6;
    }
    if let Some(ref nested) = o.tail.nested {
        if nested.tail_x != 7 {
            return 7;
        }
        if nested.tail_union.tail_y != 8 {
            return 8;
        }
    } else {
        return 7;
    }

    0
}

fn check_union_aliasing_via_flattened_names() -> i32 {
    let mut o = Outer {
        base: 0,
        inner: InnerStruct {
            a: 0,
            union_field: InnerUnion {
                u_as_int: 0,
                nested: Some(InnerNested {
                    b: 0,
                    deep_union: DeepUnion {
                        deep: 0,
                        xy: None,
                    },
                    c: 0,
                }),
                u_as_unsigned: 0,
            },
            d: 0,
        },
        tail: TailUnion {
            tail_i: 0,
            nested: Some(TailNested {
                tail_x: 0,
                tail_union: TailDeepUnion {
                    tail_y: 0,
                    tail_pq: None,
                },
            }),
        },
    };

    if let Some(ref mut nested) = o.inner.union_field.nested {
        nested.deep_union.deep = 0x11223344;
        if nested.deep_union.deep != 0x11223344 {
            return 20;
        }

        nested.deep_union.xy = Some((5, 6));
        if let Some((x, y)) = nested.deep_union.xy {
            if x != 5 {
                return 21;
            }
            if y != 6 {
                return 22;
            }
        } else {
            return盐 21;
        }
    } else {
        return 20;
    }

    if let Some(ref mut nested) = o.tail.nested {
        nested.tail_union.tail_pq = Some((40, 41));
        if let Some((p, q)) = nested.tail_union.tail_pq {
            if p != 40 {
                return 23;
            }
            if q != 41 {
                return 24;
            }
        } else {
            return 23;
        }

        nested.tail_union.tail_y = -9;
        if nested.tail_union.tail_y != -9 {
            return 25;
        }
    } else {
        return 23;
    }

    0
}

fn check_addressability() -> i32 {
    let o = Outer {
        base: 0,
        inner: InnerStruct {
            a: 0,
            union_field: InnerUnion {
                u_as_int: 0,
                nested: Some(InnerNested {
                    b: 0,
                    deep_union: DeepUnion {
                        deep: 0,
                        xy: None,
                    },
                    c: 0,
                }),
                u_as_unsigned: 0,
            },
            d: 0,
        },
        tail: TailUnion {
            tail_i: 0,
            nested: Some(TailNested {
                tail_x: 0,
                tail_union: TailDeepUnion {
                    tail_y: 0,
                    tail_pq: None,
                },
            }),
        },
    };

    let pa = &o.inner.a as *const i32 as Uptr;
    let pb = if let Some(ref nested) = o.inner.union_field.nested {
        &nested.b as *const i32 as Uptr
    } else {
        0
    };
    let pdeep = if let Some(ref nested) = o.inner.union_field.nested {
        &nested.deep_union.deep as *const i32 as Uptr
    } else {
        0
    };
    let ptx = if let Some(ref nested) = o.tail.nested {
        &nested.tail_x as *const i32 as Uptr
    } else {
        0
    };
    let pty = if let Some(ref nested) = o.tail.nested {
        &nested.tail_union.tail_y as *const i32 as Uptr
    } else {
        0
    };

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