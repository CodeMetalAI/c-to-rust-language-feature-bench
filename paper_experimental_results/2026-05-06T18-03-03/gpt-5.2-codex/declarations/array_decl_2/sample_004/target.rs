use std::process::exit;

fn sum_ptr(p: &[i32], n: i32) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i as usize];
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < 7 {
        s += a[i as usize];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

struct HoldP<'a> {
    p: &'a [i32],
}

fn main() {
    let mut y = [0i32; 7];
    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    if sum_arr7(&y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        exit(1);
    }

    let mut backing = [10i32, 20, 30];

    if sum_ptr(&backing, 3) != (10 + 20 + 30) {
        exit(2);
    }

    if mutate_through_pointer(&mut backing) != (10 + 25 + 30) {
        exit(3);
    }

    if backing[1] != 25 {
        exit(4);
    }

    {
        let t = &y;
        if t[6] != 7 {
            exit(5);
        }
    }

    {
        let hp = HoldP { p: &y };
        if hp.p[0] != 1 {
            exit(6);
        }

        let tag_index = 0usize;
        let a_start_index = 1usize;
        let _ha_tag = y[tag_index];
        let ha_a = &y[a_start_index..];

        let offset = std::mem::size_of::<i32>();
        let actual_offset = (a_start_index - tag_index) * std::mem::size_of::<i32>();

        if actual_offset != offset {
            exit(7);
        }

        if ha_a[2] != 3 {
            exit(8);
        }
    }

    if y[0] != 1 {
        exit(9);
    }

    exit(0);
}