#[derive(Clone, Copy)]
struct S1 {
    x: i32,
}

#[derive(Clone, Copy)]
struct S2 {
    x: i32,
}

type T1 = S1;
type TP1 = *mut T1;  // Rust raw pointer, but only used in safe context as per C code

type T2 = S2;
type TP2 = *mut T2;

fn takes_t1(v: T1) -> i32 {
    v.x + 1
}

fn takes_s1(v: S1) -> i32 {
    v.x + 2
}

fn takes_tp1(p: TP1) -> i32 {
    unsafe {
        // Safe because we only dereference pointers that are valid and non-null
        (*p).x += 3;
        (*p).x
    }
}

fn takes_int(v: i32) -> i32 {
    v + 4
}

fn takes_t2(v: T2) -> i32 {
    v.x + 5
}

fn main() {
    let mut a = T1 { x: 10 };
    let b = S1 { x: 20 };

    if takes_t1(a) != 11 {
        return;
    }

    if takes_s1(a) != 12 {
        return;
    }

    if takes_t1(b) != 21 {
        return;
    }

    if takes_s1(b) != 22 {
        return;
    }

    let p: TP1 = &mut a;

    if takes_tp1(p) != 13 {
        return;
    }

    if a.x != 13 {
        return;
    }

    if takes_int(a.x) != 17 {
        return;
    }

    {
        let mut q = 0;
        q += std::mem::size_of::<T1>() as i32;
        q += std::mem::size_of::<S1>() as i32;
        if q == 0 {
            return;
        }
    }

    {
        let c = T2 { x: 30 };
        let r: TP2 = &mut c;
        unsafe {
            if (*r).x != 30 {
                return;
            }
        }
        if takes_t2(c) != 35 {
            return;
        }
    }

    // Success exit
}