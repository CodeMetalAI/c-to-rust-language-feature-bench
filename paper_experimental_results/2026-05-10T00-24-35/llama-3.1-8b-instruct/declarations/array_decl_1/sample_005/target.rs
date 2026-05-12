struct Pos {
    calls: usize,
}

impl Pos {
    fn new() -> Self {
        Pos { calls: 0 }
    }

    fn pos(&mut self, x: i32) -> i32 {
        self.calls += 1;
        if x <= 0 {
            1
        } else {
            x
        }
    }
}

struct Array1<'a> {
    data: &'a [f32],
    count: usize,
}

impl<'a> Array1<'a> {
    fn new(data: &'a [f32]) -> Self {
        Array1 {
            data,
            count: data.len(),
        }
    }
}

struct Array2<'a> {
    data: &'a [f32],
    count: usize,
}

impl<'a> Array2<'a> {
    fn new(data: &'a [f32]) -> Self {
        Array2 {
            data,
            count: data.len(),
        }
    }
}

fn init_globals() {
    let mut fa = vec![0.0; 11];
    for i in 0..11 {
        fa[i] = (i as f32 + 1.0);
    }

    let mut backing = vec![0.0; 17];
    for i in 0..17 {
        backing[i] = 100.0 + (i as f32);
    }

    let mut afp = vec![&backing[i]; 17];
    for i in 0..17 {
        afp[i] = &backing[i];
    }
}

fn sum_ints_from_float(p: &[f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += (p[i] as i32);
    }
    s
}

fn sum_pointed_ints(pp: &[&f32], n: usize) -> i32 {
    let mut s = 0;
    for i in 0..n {
        s += (*pp[i] as i32);
    }
    s
}

fn takes_params(a: &[f32], afp2: &[&f32]) -> i32 {
    let sum1 = (a[0] as i32) + (a[10] as i32);
    let sum2 = (**afp2[0] as i32) + (**afp2[16] as i32);
    sum1 + sum2
}

fn main() {
    init_globals();

    let fa = Array1::new(&fa);
    let afp = Array2::new(&afp);

    let mut pos_calls = Pos::new();
    let n1 = pos_calls.pos(11);
    let n2 = pos_calls.pos(17);

    let vla1 = Array1::new(&fa.data[..n1]);
    let vla2 = Array2::new(&afp.data[..n2]);

    for i in 0..n1 {
        vla1.data[i] = fa.data[i] * 2.0;
    }

    for i in 0..n2 {
        vla2.data[i] = afp.data[i];
    }

    if pos_calls.calls != 2 {
        panic!("Expected 2 calls to pos, but got {}", pos_calls.calls);
    }

    let span = (&vla1.data[n1 - 1] as *const f32) as usize -
               (&vla1.data[0] as *const f32) as usize;
    if span != (n1 - 1) * std::mem::size_of::<f32>() {
        panic!("Incorrect span value");
    }

    if sum_ints_from_float(&vla1.data, 11) != (2 + 4 + 6 + 8 + 10 + 12 + 14 + 16 + 18 + 20 + 22) {
        panic!("Incorrect sum from vla1");
    }

    if sum_pointed_ints(&[&vla2.data[0], &vla2.data[16]], 2) != (100 + 101 + 102 + 103 + 104 + 105 + 106 + 107 + 108 + 109 + 110 + 111 + 112 + 113 + 114 + 115 + 116) {
        panic!("Incorrect sum from vla2");
    }

    if takes_params(&vla1.data, &[&vla2.data[0], &vla2.data[16]]) != ((vla1.data[0] as i32) + (vla1.data[10] as i32) + (**&vla2.data[0] as i32) + (**&vla2.data[16] as i32)) {
        panic!("Incorrect result from takes_params");
    }
}