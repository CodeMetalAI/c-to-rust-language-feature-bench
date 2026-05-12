type Usize = usize;

static mut X: *const i32 = std::ptr::null();
static mut Y: [i32; 7] = [0; 7];

static BACKING: [i32; 3] = [10, 20, 30];

#[repr(C)]
struct HoldP {
    p: *const i32,
}

#[repr(C)]
struct HoldA {
    tag: i32,
    a: [i32; 0], // Flexible array member replacement in Rust
}

fn sum_ptr(p: &[i32], n: usize) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += p[i];
        i += 1;
    }
    s
}

fn sum_arr7(a: &[i32; 7]) -> i32 {
    let mut s = 0;
    let mut i = 0;
    while i < 7 {
        s += a[i];
        i += 1;
    }
    s
}

fn mutate_through_pointer(p: &mut [i32; 3]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() {
    unsafe {
        Y = [1, 2, 3, 4, 5, 6, 7];
        X = BACKING.as_ptr();

        if sum_arr7(&Y) != (1 + 2 + 3 + 4 + 5 + 6 + 7) {
            std::process::exit(1);
        }

        let x_slice = &BACKING;
        if sum_ptr(x_slice, 3) != (10 + 20 + 30) {
            std::process::exit(2);
        }

        let mut backing_copy = BACKING;
        if mutate_through_pointer(&mut backing_copy) != (10 + 25 + 30) {
            std::process::exit(3);
        }

        if backing_copy[1] != 25 {
            std::process::exit(4);
        }

        let t = Y.as_ptr();
        if *t.add(6) != 7 {
            std::process::exit(5);
        }

        let hp = HoldP { p: Y.as_ptr() };
        if *hp.p != 1 {
            std::process::exit(6);
        }

        let ha: *const HoldA = Y.as_ptr() as *const HoldA;

        if (&(*ha).a as *const [i32; 0] as *const u8).wrapping_offset(0)
            != (ha as *const u8).wrapping_offset(std::mem::size_of::<i32>() as isize)
        {
            std::process::exit(7);
        }

        if *Y.get_unchecked(2) != 3 {
            std::process::exit(8);
        }

        if Y[0] != 1 {
            std::process::exit(9);
        }
    }
}