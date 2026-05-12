use rand::Rng;

const D0: u32 = 0;
const D1: u32 = 1;
const D2: u32 = 2;
const D3: u32 = 3;
const D4: u32 = 4;
const D5: u32 = 5;
const D6: u32 = 6;
const D7: u32 = 7;
const D8: u32 = 8;
const D9: u32 = 9;

fn pickd(n: u32) -> u32 {
    match n {
        0 => D0,
        1 => D1,
        2 => D2,
        3 => D3,
        4 => D4,
        5 => D5,
        6 => D6,
        7 => D7,
        8 => D8,
        9 => D9,
        _ => unreachable!(),
    }
}

fn vxyz(a: u32, b: u32, c: u32) -> u32 {
    pickd(a) * 100 + pickd(b) * 10 + pickd(c)
}

fn vyz(b: u32, c: u32) -> u32 {
    pickd(b) * 10 + pickd(c)
}

fn vxz(a: u32, c: u32) -> u32 {
    pickd(a) * 10 + pickd(c)
}

fn vxy(a: u32, b: u32) -> u32 {
    pickd(a) * 10 + pickd(b)
}

struct Rng {
    state: u32,
}

impl Rng {
    fn new() -> Self {
        Rng { state: 2463534242 }
    }

    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn pick_1_9(&mut self) -> u32 {
        self.next() % 9 + 1
    }

    fn pick_0_9(&mut self) -> u32 {
        self.next() % 10
    }
}

fn main() {
    let mut rng = Rng::new();

    for _ in 0..50 {
        let a = rng.pick_1_9();
        let b = rng.pick_0_9();
        let c = rng.pick_0_9();

        let v1 = vxyz(a, b, c);
        if v1 != 100 * a + 10 * b + c {
            std::process::exit(1);
        }

        let b2 = rng.pick_1_9();
        let c2 = rng.pick_0_9();
        let v2 = vyz(b2, c2);
        if v2 != 10 * b2 + c2 {
            std::process::exit(2);
        }

        let a3 = rng.pick_1_9();
        let c3 = rng.pick_0_9();
        let v3 = vxz(a3, c3);
        if v3 != 10 * a3 + c3 {
            std::process::exit(3);
        }

        let a4 = rng.pick_1_9();
        let b4 = rng.pick_0_9();
        let v4 = vxy(a4, b4);
        if v4 != 10 * a4 + b4 {
            std::process::exit(4);
        }
    }
}