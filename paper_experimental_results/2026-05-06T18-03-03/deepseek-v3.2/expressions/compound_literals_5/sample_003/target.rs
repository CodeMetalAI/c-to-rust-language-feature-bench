fn main() {
    let s1 = "/tmp/fileXXXXXX";
    
    let mut s2_array = ["/tmp/fileXXXXXX"];
    let s2 = s2_array[0].as_mut_ptr() as *mut char;
    let s2_slice = unsafe { std::slice::from_raw_parts_mut(s2, s1.len()) };
    
    let s3_array = ["/tmp/fileXXXXXX"];
    let s3 = s3_array[0].as_ptr() as *const char;
    let s3_slice = unsafe { std::slice::from_raw_parts(s3, s1.len()) };
    
    if s1.chars().next().unwrap() != '/' || 
       s2_slice[0] != '/' || 
       s3_slice[0] != '/' {
        std::process::exit(1);
    }
    
    if s1.chars().nth(1).unwrap() != 't' || 
       s2_slice[1] != 't' || 
       s3_slice[1] != 't' {
        std::process::exit(2);
    }
    
    s2_slice[0] = 'X';
    if s2_slice[0] != 'X' {
        std::process::exit(3);
    }
    
    std::process::exit(0);
}