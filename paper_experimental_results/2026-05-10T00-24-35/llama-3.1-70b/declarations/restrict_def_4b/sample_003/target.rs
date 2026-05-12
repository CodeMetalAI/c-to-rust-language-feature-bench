fn main() {
    let mut h = 0x9e3779b97f4a7c15u64;

    struct Vector {
        n: usize,
        v: Vec<f32>,
    }

    impl Vector {
        fn new(n: usize) -> Option<Self> {
            let v = vec![0.0; n];
            h ^= (v.as_ptr() as *const _ as u64) ^ 0x9e3779b97f4a7c15u64;
            h *= 0x5851f42d4c957f2du64;
            Some(Vector { n, v })
        }
    }

    fn fill_vec(p: &mut [f32], base: f32) {
        for (i, x) in p.iter_mut().enumerate() {
            *x = base + i as f32;
        }
    }

    fn sum_vec(p: &[f32]) -> f32 {
        p.iter().sum()
    }

    fn close_enough(x: f32, y: f32) -> bool {
        let d = x - y;
        (d < 0.0).then(|| -d).unwrap_or(d) < 0.0001
    }

    let a = Vector::new(16).unwrap();
    let b = Vector::new(16).unwrap();

    if a.v.is_empty() {
        return 1;
    }
    if b.v.is_empty() {
        return 2;
    }

    if a.v.as_ptr() == b.v.as_ptr() {
        return 3;
    }

    fill_vec(&mut a.v, 1.0);
    fill_vec(&mut b.v, 100.0);

    if!close_enough(sum_vec(&a.v), 1.0 * 16.0 + (15 * 16) as f32 / 2.0) {
        return 4;
    }

    if!close_enough(
        sum_vec(&b.v),
        100.0 * 16.0 + (15 * 16) as f32 / 2.0,
    ) {
        return 5;
    }

    if (h & 1) == 0 {
        return 6;
    }

    println!();
}