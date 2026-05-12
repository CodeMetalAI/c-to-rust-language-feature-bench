fn main() -> std::process::ExitCode {
    let mut arr = [0; 3];

    let p: *mut i32 = arr.as_mut_ptr();
    
    unsafe {
        *p.add(0) = 1;
        *p.add(1) = 99;
        *p.add(2) = 3;
    }

    if arr[1] != 99 {
        return std::process::ExitCode::FAILURE;
    }

    std::process::ExitCode::SUCCESS
}