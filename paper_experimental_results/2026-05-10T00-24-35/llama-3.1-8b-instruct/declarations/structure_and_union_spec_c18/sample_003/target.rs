// Define unsigned types
#[allow(unused)]
type U8 = u8;
#[allow(unused)]
type U32 = u32;

// Define structure Packet
#[repr(C)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: [U32],
}

// Define function to compute sum
fn compute_sum(n: U32) -> U32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1u32) * 3u32 + 1u32;
        i += 1;
    }
    s
}

// Define test function for non-empty object
fn test_nonempty_object() -> i32 {
    // Define constant N
    const N: usize = 7;

    // Define union to store Packet and bytes
    #[repr(C)]
    struct Storage {
        raw: [U8; N * 4 + 40],
        align: U32,
    }

    // Create Storage and cast it to Packet
    let p: &mut Packet = unsafe { &mut *(Storage::raw_mut() as *mut Packet) };

    // Initialize Packet
    p.tag = 0xA1B2C3D4u32;
    p.n = N as U32;

    {
        // Compute offset of data field in Packet
        let off = core::mem::size_of::<Packet>() as usize + N * core::mem::size_of::<U32>();

        // Get pointer to data field using references
        let expected: *mut U8 = Storage::raw_mut().as_mut_ptr().add(off);
        let got1: *mut U8 = (&p.data).as_mut_ptr();
        let got2: *mut U8 = (&*p).data.as_mut_ptr();

        // Check if pointers are equal
        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }

    // Initialize data field and check it
    for i in 0..N {
        let v = (i + 1u32) * 3u32 + 1u32;
        p.data[i] = v;
        (*p).data[i] += 0u32;
    }

    // Initialize sum field and check it
    p.sum = 0;
    for i in 0..N {
        p.sum += p.data[i];
    }

    // Check if sum matches computed sum
    if p.sum != compute_sum(N) {
        return 3;
    }

    // Check if size of Packet is at least offset of data field
    if core::mem::size_of::<Packet>() as usize < OFFSETOF(Packet, data) {
        return 4;
    }

    // Return 0 on success
    0
}

// Define test function for zero-element object
fn test_zero_element_object() -> i32 {
    // Define union to store Packet and bytes
    #[repr(C)]
    struct Storage {
        raw: [U8; core::mem::size_of::<Packet>()],
        align: U32,
    }

    // Create Storage and cast it to Packet
    let p: &mut Packet = unsafe { &mut *(Storage::raw_mut() as *mut Packet) };

    // Initialize Packet
    p.tag = 0u32;
    p.n = 0u32;
    p.sum = 0u32;

    {
        // Compute offset of data field in Packet
        let off = core::mem::size_of::<Packet>() as usize;

        // Get pointer to data field using references
        let expected: *mut U8 = Storage::raw_mut().as_mut_ptr().add(off);
        let got1: *mut U8 = (&p.data).as_mut_ptr();
        let got2: *mut U8 = (&*p).data.as_mut_ptr();

        // Check if pointers are equal
        if got1 != expected {
            return 10;
        }
        if got2 != expected {
            return 11;
        }
    }

    // Return 0 on success
    0
}

fn main() {
    let r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }

    let r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}

// Macro to compute offset of a field in a struct
macro_rules! OFFSETOF {
    ($type:ty, $member:ident) => {
        core::mem::size_of::<$type>() as usize + core::mem::offset_of!($type, $member)
    };
}