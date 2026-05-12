fn main() -> i32 {
    let mut y: [i32; 7] = [10, 20, 30, 40, 50, 60, 70];

    let x: *const i32 = y.as_ptr();

    let backing: [i32; 3] = [10, 20, 30];
    let x2: *const i32 = backing.as_ptr();

    struct HoldP {
        p: *const i32,
    }

    struct HoldA {
        tag: i32,
        a: [i32; 7],
    }

    mut hp = HoldP { p: y.as_ptr() };
    let mut ha: *mut HoldA = std::ptr::null_mut();

    unsafe {
        let sum_ptr_result = sum_ptr(x, 7);
        let sum_arr_result = sum_arr(&y);
        if sum_ptr_result != (10 + 20 + 30 + 40 + 50 + 60 + 70) {
            return 1;
        }
        if sum_arr_result != (10 + 20 + 30 + 40 + 50 + 60 + 70) {
            return 2;
        }

        let mut temp = y;
        mutate_through_pointer(temp.as_mut_ptr());
        if temp[1] != 25 {
            return 3;
        }

        if backing[1] != 20 {
            return 4;
        }

        if hp.p != y.as_ptr() {
            return 5;
        }

        ha = y.as_ptr() as *mut HoldA;
        if (*ha).tag != 10 {
            return 6;
        }
        if (*ha).a[2] != 30 {
            return 7;
        }

        if y[0] != 10 {
            return 8;
        }
    }

    0
}

fn sum_ptr(p: *const i32, n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        unsafe {
            s += *p.add(i);
        }
        i += 1;
    }
    s
}

fn sum_arr(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    for &val in a {
        s += val;
    }
    s
}

fn mutate_through_pointer(p: *mut i32) -> i32 {
    unsafe {
        *p.add(1) += 5;
        *p
    }
}