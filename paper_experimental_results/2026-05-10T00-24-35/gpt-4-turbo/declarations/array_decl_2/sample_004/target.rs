fn main() {
    let mut y = [0; 7];
    let mut backing = [10, 20, 30];
    let x = &mut backing;

    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        std::process::exit(1);
    }

    if sum_ptr(x, 3) != (10 + 20 + 30) {
        std::process::exit(2);
    }

    if mutate_through_pointer(x) != (10 + 25 + 30) {
        std::process::exit(3);
    }

    if backing[1] != 25 {
        std::process::exit(4);
    }

    if y[6] != 7 {
        std::process::exit(5);
    }

    {
        let hp = HoldP { p: &y };
        if hp.p[0] != 1 {
            std::process::exit(6);
        }

        let ha = unsafe { &*(y.as_ptr() as *const HoldA) };
        let ha_a_offset = offset_of!(HoldA, a) as isize;
        let ha_a_ptr = unsafe { (ha as *const _ as *const u8).offset(ha_a_offset) as *const _ };

        if ha_a_ptr != ha.a.as_ptr() {
            std::process::exit(7);
        }

        if ha.a[2] != 3 {
            std::process::exit(8);
        }
    }

    if y[0] != 1 {
        std::process::exit(9);
    }
}

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for &val in p.iter().take(n) {
        s += val;
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    a.iter().sum()
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p.iter().sum()
}

struct HoldP<'a> {
    p: &'a [i32],
}

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32],
}

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe {
            let instance = std::mem::MaybeUninit::<$ty>::uninit();
            let base_ptr = instance.as_ptr() as *const u8;
            let field_ptr = &(*instance.as_ptr()).$field as *const _ as *const u8;
            field_ptr.offset_from(base_ptr) as usize
        }
    };
}