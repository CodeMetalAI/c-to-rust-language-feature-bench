// array_decl_2.rs

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 7],
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += *p.offset(i as isize);
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < a.len() {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() {
    let mut y = [1, 2, 3, 4, 5, 6, 7];

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        panic!("sum_arr7 failed");
    }

    let backing = [10, 20, 30];
    let x = &backing;
    if sum_ptr(x.as_ptr(), 3) != (10 + 20 + 30) {
        panic!("sum_ptr failed");
    }

    if mutate_through_pointer(&mut backing) != (10 + 25 + 30) {
        panic!("mutate_through_pointer failed");
    }

    if backing[1] != 25 {
        panic!("backing[1] failed");
    }

    {
        let t = &y;
        if t[6] != 7 {
            panic!("t[6] failed");
        }
    }

    {
        let hp = HoldP { p: y.as_ptr() };
        if *hp.p.offset(0) != 1 {
            panic!("hp.p[0] failed");
        }

        let ha = unsafe { &*(y.as_ptr() as *const HoldA) };
        if (ha as *const _ as *const u8) != (ha as *const _ as *const u8).offset(OFFSETOF!(HoldA, a)) {
            panic!("ha->a offset failed");
        }

        if ha.a[2] != 3 {
            panic!("ha->a[2] failed");
        }
    }

    if y[0] != 1 {
        panic!("y[0] failed");
    }
}