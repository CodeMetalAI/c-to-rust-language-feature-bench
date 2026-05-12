fn main() {
    let s1 = "/tmp/fileXXXXXX";

    let mut s2_array = ["/tmp/fileXXXXXX"];
    let s2 = s2_array[0].as_mut_ptr() as *mut char;
    let s2_slice = unsafe { std::slice::from_raw_parts_mut(s2, s1.len()) };
    let s2_str = unsafe { std::str::from_utf8_unchecked_mut(std::mem::transmute::<&mut [char], &mut [u8]>(s2_slice)) };

    let s3_array = ["/tmp/fileXXXXXX"];
    let s3 = s3_array[0].as_ptr() as *const char;
    let s3_slice = unsafe { std::slice::from_raw_parts(s3, s1.len()) };
    let s3_str = unsafe { std::str::from_utf8_unchecked(std::mem::transmute::<&[char], &[u8]>(s3_slice)) };

    if s1.as_bytes()[0] != b'/' || s2_str.as_bytes()[0] != b'/' || s3_str.as_bytes()[0] != b'/' {
        std::process::exit(1);
    }
    if s1.as_bytes()[1] != b't' || s2_str.as_bytes()[1] != b't' || s3_str.as_bytes()[1] != b't' {
        std::process::exit(2);
    }

    unsafe {
        *s2 = 'X';
    }
    if s2_str.as_bytes()[0] != b'X' {
        std::process::exit(3);
    }

    std::process::exit(0);
}