use std::rc::Rc;
use std::cell::RefCell;

struct Pool {
    data: Vec<f32>,
    off: usize,
    h: u64,
}

impl Pool {
    fn new() -> Self {
        Self {
            data: vec![0.0; 2048],
            off: 0,
            h: 0x9e3779b97f4a7c15u64,
        }
    }

    fn alloc_floats(&mut self, n: usize) -> Option<usize> {
        if self.off + n > 2048 {
            return None;
        }
        let start = self.off;
        self.off += n;
        self.h ^= ((start * 4) as u64).wrapping_add(0x9e3779b97f4a7c15u64);
        self.h = self.h.wrapping_mul(0x5851f42d4c957f2d);
        Some(start)
    }
}

struct Vector {
    n: i32,
    offset: usize,
    pool: Rc<RefCell<Pool>>,
}

fn new_vector(n: i32, pool: &Rc<RefCell<Pool>>) -> Option<Vector> {
    let offset = pool.borrow_mut().alloc_floats(n as usize)?;
    Some(Vector {
        n,
        offset,
        pool: pool.clone(),
    })
}

fn fill_vec(v: &Vector, base: f32) {
    let mut pool = v.pool.borrow_mut();
    let p = &mut pool.data[v.offset..v.offset + v.n as usize];
    for i in 0..v.n {
        p[i as usize] = base + i as f32;
    }
}

fn sum_vec(v: &Vector) -> f32 {
    let pool = v.pool.borrow();
    let p = &pool.data[v.offset..v.offset + v.n as usize];
    p.iter().sum()
}

fn close_enough(x: f32, y: f32) -> bool {
    let d = x - y;
    let d = if d < 0.0 { -d } else { d };
    d < 0.0001
}

fn main() -> i32 {
    let pool = Rc::new(RefCell::new(Pool::new()));
    let a = match new_vector(16, &pool) {
        Some(v) => v,
        None => return 1,
    };
    let b = match new_vector(16, &pool) {
        Some(v) => v,
        None => return 2,
    };

    if a.offset == b.offset {
        return 3;
    }

    fill_vec(&a, 1.0);
    fill_vec(&b, 100.0);

    if !close_enough(sum_vec(&a), 1.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 4;
    }

    if !close_enough(sum_vec(&b), 100.0 * 16.0 + (15.0 * 16.0) / 2.0) {
        return 5;
    }

    let h = pool.borrow().h;
    if (h & 1) == 0 {
        return 6;
    }

    0
}