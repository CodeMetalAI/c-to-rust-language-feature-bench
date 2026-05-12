fn main() {
    #[derive(Debug, Default, Copy, Clone)]
    struct S1 {
        x: i32,
    }

    type T1 = S1;
    type TP1 = *mut T1;

    fn takes_t1(v: T1) -> i32 {
        v.x + 1
    }

    fn takes_s1(v: S1) -> i32 {
        v.x + 2
    }

    fn takes_tp1(p: &mut T1) -> i32 {
        p.x += 3;
        p.x
    }

    fn takes_int(v: i32) -> i32 {
        v + 4
    }

    #[derive(Debug, Default, Copy, Clone)]
    struct S2 {
        x: i32,
    }

    type T2 = S2;
    type TP2 = *mut T2;

    fn takes_t2(v: T2) -> i32 {
        v.x + 5
    }

    let mut a = T1 { x: 10 };
    let b = S1 { x: 20 };
    let mut p = &mut a;

    if takes_t1(a)!= 11 {
        return;
    }

    if takes_s1(a)!= 12 {
        return;
    }

    if takes_t1(b)!= 21 {
        return;
    }

    if takes_s1(b)!= 22 {
        return;
    }

    if takes_tp1(&mut a)!= 13 {
        return;
    }

    if a.x!= 13 {
        return;
    }

    if takes_int(a.x)!= 17 {
        return;
    }

    {
        let mut q = 0;
        q += std::mem::size_of::<T1>();
        q += std::mem::size_of::<S1>();
        if q == 0 {
            return;
        }
    }

    {
        let mut c = T2 { x: 30 };
        let r = &mut c;
        if r.x!= 30 {
            return;
        }
        if takes_t2(c)!= 35 {
            return;
        }
    }
}