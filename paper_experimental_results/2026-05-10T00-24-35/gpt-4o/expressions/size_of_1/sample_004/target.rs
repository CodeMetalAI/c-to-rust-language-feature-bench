/*
 * size_of_1
 */

fn alloc(n: usize) -> Option<Box<[u8]>> {
    if n == 0 {
        None
    } else {
        Some(vec![0; n].into_boxed_slice())
    }
}

fn main() {
    let dp: Option<Box<[u8]>> = alloc(std::mem::size_of::<f64>());

    if dp.is_none() {
        std::process::exit(1);
    }

    let mut dp = dp.unwrap();
    let dp_ptr = dp.as_mut_ptr() as *mut f64;

    unsafe {
        *dp_ptr = 3.14;
        if *dp_ptr != 3.14 {
            std::process::exit(2);
        }
    }

    std::process::exit(0);
}