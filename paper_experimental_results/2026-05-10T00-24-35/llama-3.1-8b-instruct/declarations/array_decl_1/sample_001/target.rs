use std::mem::size_of;

struct FloatArray {
    data: [f32; 11],
}

impl FloatArray {
    fn new() -> Self {
        Self {
            data: [1.0; 11].into(),
        }
    }

    fn get(&self, index: usize) -> f32 {
        self.data[index]
    }
}

struct FloatPtrArray {
    data: [f32; 17],
    pointers: *const [f32],
}

impl FloatPtrArray {
    fn new() -> Self {
        let backing = [100.0; 17].into();
        let pointers = &backing as *const _;
        Self {
            data: backing,
            pointers,
        }
    }

    fn get(&self, index: usize) -> *const f32 {
        &self.data[index] as *const _
    }
}

fn pos<T>(x: T) -> usize
where
    T: Copy + std::ops::AddAssign<usize>,
{
    let mut calls = 0;
    calls += 1;
    if x <= 0 {
        return 1;
    }
    x
}

fn sum_ints_from_float<const N: usize>(p: *const f32) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < N {
        sum += (p as *const i32).read();
        i += 1;
    }
    sum
}

fn sum_pointed_ints<const N: usize>(pp: *const *const f32) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    while i < N {
        sum += ((*pp as *const f32).read() as *const i32).read();
        i += 1;
    }
    sum
}

fn takes_params<const N1: usize, const N2: usize>(
    a: *const f32,
    afp2: *const *const f32,
) -> i32 {
    let s1 = ((*(a as *const i32)).read() +
             (*(a.offset((N1 - 1) as isize) as *const i32)).read()) as i32;
    let s2 = ((*(afp2 as *const i32)).read() +
             (*(afp2.offset((N2 - 1) as isize) as *const i32)).read()) as i32;
    s1 + s2
}

fn main() {
    let fa = FloatArray::new();
    let afp = FloatPtrArray::new();

    assert_eq!(fa.get(0), 1.0);
    assert_eq!(fa.get(10), 11.0);

    assert_eq!((*afp.get(0) as *const i32).read(), 100);
    assert_eq!((*afp.get(16) as *const i32).read(), 116);

    let n1 = pos(11);
    let n2 = pos(17);
    let mut vla1 = vec![0.0; n1];
    let mut vla2 = vec![0.0; n2];

    for i in 0..n1 {
        vla1[i] = fa.get(i) * 2.0;
    }

    for i in 0..n2 {
        vla2[i] = afp.get(i);
    }

    assert_eq!(std::mem::size_of::<[f32; 11]>() * (n1 - 1), std::mem::size_of::<[f32; n1]>());
    assert_eq!(sum_ints_from_float(vla1.as_ptr()), 2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22);
    assert_eq!(sum_pointed_ints(vla2.as_ptr()), 100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116);
    assert_eq!(takes_params(vla1.as_ptr(), vla2.as_ptr()), 2 + 22 + 116);
}