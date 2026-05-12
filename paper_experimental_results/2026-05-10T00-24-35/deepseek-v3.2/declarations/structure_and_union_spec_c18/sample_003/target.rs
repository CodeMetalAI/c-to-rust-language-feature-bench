fn main() {
    let result = test_nonempty_object();
    if result != 0 {
        std::process::exit(result);
    }

    let result = test_zero_element_object();
    if result != 0 {
        std::process::exit(result);
    }

    std::process::exit(0);
}

type u8 = std::os::raw::c_uchar;
type u32 = std::os::raw::c_uint;

const OFFSETOF: usize = std::mem::size_of::<u32>() * 2;

struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0],
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1) * 3 + 1;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: u32 = 7;

    let mut storage: [u8; OFFSETOF + N as usize * std::mem::size_of::<u32>() + 32] = [0; OFFSETOF + N as usize * std::mem::size_of::<u32>() + 32];

    let p_ptr = storage.as_mut_ptr() as *mut Packet;
    let p = unsafe { &mut *p_ptr };

    p.tag = 0xA1B2C3D4;
    p.n = N;

    let expected = storage.as_mut_ptr().add(OFFSETOF);
    let got1 = p.data.as_ptr() as *mut u8;
    let got2 = (&p.data).as_ptr() as *mut u8;

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return发2;
    }

    let mut i = 0;
    while i < p.n {
        let v = (i + 1) * 3 + 1;
        unsafe {
            *((p.data.as_ptr() as *mut u32).add(i as usize)) = v;
            *(((&p.data).as_ptr() as *mut u32).add(i as usize)) += 0;
        }
        i += 1;
    }

    p.sum = 0;
    let mut i = 0;
    while i < p.n {
        unsafe {
            p.sum += *((p.data.as_ptr() as *mut u32).add(i as usize));
        }
        i += 1;
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < OFFSETOF {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [u8; OFFSETOF] = [0; OFFSETOF];

    let p_ptr = storage.as_mut_ptr() as *mut Packet;
    let p = unsafe { &mut *p_ptr };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    let expected = storage.as_mut_ptr().add(OFFSETOF);
    let got1 = p.data.as_ptr() as *mut u8;
    let got2 = (&p.data).as_ptr() as *mut u8;

    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
    }

    0
}