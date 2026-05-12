use std::mem;

fn main() {
    let mut y = [0; 7];
    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    assert_eq!(sum_arr(&y), 28);

    let mut x = [10, 20, 30];
    assert_eq!(sum_ptr(&x), 60);

    let result = mutate_through_pointer(&mut x);
    assert_eq!(result, 55);

    assert_eq!(x[1], 25);

    let t = &y;
    assert_eq!(t[6], 7);

    {
        let hp = HoldP { p: &y };
        assert_eq!(hp.p[0], 1);

        let ha: &HoldA = &*(y as *const [i32] as *const HoldA);
        assert_eq!((ha as *const _ as usize + mem::offset_of!(HoldA, a) as usize) as *const usize,
                  (&ha.a[0] as *const _ as usize));
        assert_eq!(ha.a[2], 3);
    }

    assert_eq!(y[0], 1);
}

struct HoldP {
    p: *const i32,
}

struct HoldA {
    tag: i32,
    a: [i32; 7],
}

fn sum_arr(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn sum_ptr(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn mutate_through_pointer(p: &mut [i32]) -> i32 {
    p[1] += 5;
    p[0] + p[1] + p[2]
}