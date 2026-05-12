use std::sync::atomic::AtomicPtr;

static mut F_TABLE: [AtomicPtr<()>; 4] = [
    AtomicPtr::new(std::ptr::null_mut()),
    AtomicPtr::new(std::ptr::null_mut()),
   AtomicPtr::new(std::ptr::null_mut()),
    AtomicPtr::new(std::ptr::null_mut()),
];

fn f<const N: usize>(p: *const()) {
    let ptr = p as *const u8 as *mut();
    F_TABLE[N].store(ptr, std::sync::atomic::Ordering::Relaxed);
}

fn main() {
    let mut c_v: usize = 42;
    let mut c_ip: usize = 0;
    let mut v_ip: AtomicUsize::new(0);
    let mut ip: usize = 0;
    let mut c_c: u8 = b'A' as u8;

    f::<0>(&mut c_v as *mut usize as *const());
    f::<1>(&mut c_ip as *mut usize as *const());

    f::<2>(&v_ip as *const AtomicUsize as *const());
    f::<2>(std::ptr::null());

    f::<3>(&c_ip as *const usize as *const());
    f::<3>(&v_ip as *const AtomicUsize as *const());

    f::<0>(&v_ip as *const AtomicUsize as *const());
    f::<0>(std::ptr::null());

    f::<1>(&v_ip as *const AtomicUsize as *const());
    f::<1>(std::ptr::null());

    f::<4>(&ip作为 *const usize作为 *const());
    f::<4>(&c_ip作为 *const usize作为 *const());

    f::<0>(&v_ip作为 *const AtomicUsize作为 *const());
    f::<0>(&ip作为 *const usize作为 *const());

    f::<0>(&c_ip作为 *const usize作为 *const());
    f::<0>(std::ptr::null());

    返回 0；
}