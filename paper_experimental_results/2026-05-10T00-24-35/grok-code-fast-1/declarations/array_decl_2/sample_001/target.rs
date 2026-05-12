use std::mem;

struct HoldP<'a> {
    p: &'a mut i32,
}

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    p.iter().take(n).sum()
}

fn sum_arr7(a: &[i32]) -> i32 {
    a.iter().take(7).sum()
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() {
    let mut y: Vec<i32> = vec![0; 7];
    for i in 0..7 {
        y[i] = (i + 1) as i32;
    }

    let mut backing: Vec<i32> = vec![10, 20, 30];
    let x = &mut backing[..];

    if sum_arr7(&y) != 28 {
        std::process::exit(1);
    }

    if sum_ptr(x, 3) != 60 {
        std::process::exit(2);
    }

    if mutate_through_pointer(x) != 65 {
        std::process::exit(3);
    }

    if backing[1] != 25 {
        std::process::exit(4);
    }

    let t = &y;
    if t[6] != 7 {
        std::process::exit(5);
    }

    {
        let hp = HoldP { p: &mut y[0] };
        if *hp.p != 1 {
            std::process::exit(6);
        }

        let ha_tag = y[0];
        let ha_a = &y[1..];
        let offset_a = mem::size_of::<i32>();
        unsafe {
            if ha_a.as_ptr() as *const u8 != y.as_ptr() as *const u8.add(offset_a) {
                std::process::exit(7);
            }
        }
        if ha_a[2] != 3 {
            std::process::exit(8);
        }
    }

    if y[0] != 1 {
        std::process::exit(9);
    }

    std::process::exit(0);
}