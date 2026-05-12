// array_decl_2.rs
type Usize = usize;

static mut X: *const i32 = std::ptr::null();
static mut Y: [i32; 7] = [0; 7];

static BACKING: [i32; 3] = [10, 20, 30];

fn offsetof<T>(member: fn(&T) -> &i32) -> Usize {
    let base = std::ptr::null::<T>();
    let member_ptr = member(unsafe { &*base });
    (member_ptr as *const i32 as Usize) - (base as *const T as Usize)
}

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 0],
}

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    for i in 0..7 {
        s += a[i];
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() -> i32 {
    unsafe {
        Y = [1, 2, 3, 4, 5, 6, 7];
        X = BACKING.as_ptr();

        if sum_arr7(&Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            return 1;
        }

        if sum_ptr(&BACKING, 3) != (10 + 20 + 30) {
            return 2;
        }

        let mut backing_mut = BACKING;
        if mutate_through_pointer(&mut backing_mut) != (10 + 25 + 30) {
            return 3;
        }

        if backing_mut[1] != 25 {
            return 4;
        }

        {
            let t = &Y;
            if t[6] != 7 {
                return 5;
            }
        }

        {
            let hp = HoldP { p: Y.as_ptr() };
            if unsafe { *hp.p } != 1 {
                return 6;
            }

            let ha = &Y as *const i32 as *const HoldA;

            if (ha as *const u8).wrapping_add(offsetof(|ha: &HoldA| &ha.a[0]))
                != (&Y as *const i32 as *const u8)
            {
                return 7;
            }

            if unsafe { (*ha).a[2] } != 3 {
                return 8;
            }
        }

        if Y[0] != 1 {
            return 9;
        }
    }

    0
}