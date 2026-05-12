struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32],
}

fn sum_ptr(p: &[i32]) -> i32 {
    p.iter().sum()
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    a.iter().sum()
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p.iter().sum()
}

fn main() {
    let mut y = [0; 7];
    let mut backing = [10, 20, 30];
    let x = &backing as *const _ as *const i32;

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

    unsafe {
        if sum_ptr(std::slice::from_raw_parts(x, 3)) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        let x_mut = x as *mut i32;
        if mutate_through_pointer(std::slice::from_raw_parts_mut(x_mut, 3)) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if *x_mut.add(1) != 25 {
            std::process::exit(4);
        }

        let t = y.as_ptr();
        if *t.add(6) != 7 {
            std::process::exit(5);
        }

        let hp = HoldP { p: y.as_ptr() };
        if *hp.p != 1 {
            std::process::exit(6);
        }

        let ha = &y as *const _ as *const HoldA;
        let ha_a_ptr = ha as *const u8;

        let a_offset = std::mem::size_of::<i32>();
        if ha_a_ptr.add(a_offset) != ha_a_ptr.add(a_offset) {
            std::process::exit(7);
        }

        if *(*ha).a.get_unchecked(2) != 3 {
            std::process::exit(8);
        }
    }

    if y[0] != 1 {
        std::process::exit(9);
    }
}