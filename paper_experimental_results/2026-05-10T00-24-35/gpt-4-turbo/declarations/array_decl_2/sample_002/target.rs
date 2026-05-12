static mut BACKING: [i32; 3] = [10, 20, 30];

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += p[i];
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

fn main() {
    let mut y: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        std::process::exit(1);
    }

    unsafe {
        if sum_ptr(&BACKING, 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        if mutate_through_pointer(&mut BACKING) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if BACKING[1] != 25 {
            std::process::exit(4);
        }
    }

    if y[6] != 7 {
        std::process::exit(5);
    }

    let hp = y.as_ptr();
    if unsafe { *hp } != 1 {
        std::process::exit(6);
    }

    let ha = unsafe { &*(y.as_ptr() as *const HoldA) };

    unsafe {
        let offset_ha_a = &(*ha).a as *const _ as usize - ha as *const _ as usize;
        let expected_offset = std::mem::size_of::<i32>(); // size of `tag` in `HoldA`
        if offset_ha_a != expected_offset {
            std::process::exit(7);
        }
    }

    if ha.a[2] != 3 {
        std::process::exit(8);
    }

    if y[0] != 1 {
        std::process::exit(9);
    }
}

struct HoldA<'a> {
    tag: i32,
    a: &'a [i32],
}