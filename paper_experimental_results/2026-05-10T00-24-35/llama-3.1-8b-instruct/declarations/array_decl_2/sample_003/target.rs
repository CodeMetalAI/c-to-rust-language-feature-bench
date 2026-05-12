use std::mem;

const X: [usize; 3] = [10, 20, 30];

fn sum_slice<T>(slice: &[T]) -> T
where
    T: Copy + std::ops::Add<Output = T> + std::ops::AddAssign,
{
    slice.iter().fold(T::default(), |acc, &x| acc + x)
}

fn mutate_through_pointer(p: &mut [usize]) -> usize {
    p[1] += 5;
    p[0] + p[1] + p[2]
}

fn main() {
    let mut y: [usize; 7] = [0; 7];
    y[0] = 1;
    y[1] = 2;
    y[2] = 3;
    y[3] = 4;
    y[4] = 5;
    y[5] = 6;
    y[6] = 7;

    if sum_slice(&y)!= (1 + 2 + 3 + 4 + 5 + 6 + 7) {
        panic!("sum_slice failed");
    }

    if sum_slice(&X)!= (10 + 20 + 30) {
        panic!("sum_slice failed");
    }

    if mutate_through_pointer(&mut X)!= (10 + 25 + 30) {
        panic!("mutate_through_pointer failed");
    }

    if X[1]!= 25 {
        panic!("X[1] failed");
    }

    {
        let t = &y;
        if t[6]!= 7 {
            panic!("t[6] failed");
        }
    }

    {
        let mut hp: [usize; 1] = [0; 1];
        let mut ha: *mut [usize; 1] = std::ptr::null_mut();

        hp[0] = y[0];

        if hp[0]!= 1 {
            panic!("hp[0] failed");
        }

        ha = unsafe { &mut *std::ptr::null_mut() as *mut [usize; 1] };

        if std::mem::size_of::<[usize; 1]>() as *const _!= std::mem::size_of::<usize>() as *const _ + &ha as *const _ {
            panic!("OFFSETOF failed");
        }

        ha = unsafe { &mut *(ha as *mut usize as *mut [usize; 1]) };

        if ha[0]!= 1 {
            panic!("ha[0] failed");
        }

        if ha[1]!= 3 {
            panic!("ha[1] failed");
        }
    }

    if y[0]!= 1 {
        panic!("y[0] failed");
    }
}