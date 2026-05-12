#[allow(unused)]
enum ResultCode {
    NonemptyObjectNonempty = 1,
    NonemptyObjectNonempty2 = 2,
    NonemptyObjectMismatch = 3,
    NonemptyObjectAlignment = 4,
    ZeroElementObjectNonempty = 10,
    ZeroElementObjectNonempty2 = 11,
    Success = 0,
}

struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 7],
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

fn test_nonempty_object() -> ResultCode {
    let mut storage = [0u8; 7 * 4 + 4 + 32];
    let p = &mut Packet {
        tag: 0xA1B2C3D4,
        n: 7,
        sum: 0,
        data: [0; 7],
    };

    let offset = std::mem::size_of::<Packet>() as usize + 32;
    let expected = &storage[offset..offset + std::mem::size_of::<[u32; 7]>()];
    let got1 = unsafe { &*(p as *const Packet as *const [u32; 7]) };
    let got2 = unsafe { &*((p as *const Packet).offset(0) as *const [u32; 7]) };

    if &got1[..] != expected {
        return ResultCode::NonemptyObjectNonempty;
    }
    if &got2[..] != expected {
        return ResultCode::NonemptyObjectNonempty2;
    }

    let mut i = 0;
    while i < p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i] = v;
        unsafe { (*p).data[i] += 0; }
        i += 1;
    }

    p.sum = 0;
    i = 0;
    while i < p.n {
        p.sum += p.data[i];
        i += 1;
    }

    if p.sum != compute_sum(p.n) {
        return ResultCode::NonemptyObjectMismatch;
    }

    if std::mem::size_of::<Packet>() as usize < offset {
        return ResultCode::NonemptyObjectAlignment;
    }

    ResultCode::Success
}

fn test_zero_element_object() -> ResultCode {
    let mut storage = [0u8; std::mem::size_of::<Packet>()];
    let p = &mut Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: [0; 0],
    };

    let offset = std::mem::size_of::<Packet>();
    let expected = &storage[offset..offset + std::mem::size_of::<[u32; 0]()>()];
    let got1 = unsafe { &*(p as *const Packet as *const [u32; 0]) };
    let got2 = unsafe { &*((p as *const Packet).offset(0) as *const [u32; 0]) };

    if &got1[..] != expected {
        return ResultCode::ZeroElementObjectNonempty;
    }
    if &got2[..] != expected {
        return ResultCode::ZeroElementObjectNonempty2;
    }

    ResultCode::Success
}

fn main() {
    let r = test_nonempty_object();
    if r != ResultCode::Success {
        panic!("test_nonempty_object() failed: {:?}", r);
    }

    let r = test_zero_element_object();
    if r != ResultCode::Success {
        panic!("test_zero_element_object() failed: {:?}", r);
    }
}