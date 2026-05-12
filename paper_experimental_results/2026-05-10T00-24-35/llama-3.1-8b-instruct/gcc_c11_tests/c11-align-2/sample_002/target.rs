fn main() {
    let s1 = stringify!(alignas);
    let s2 = stringify!(alignof);
    let s3 = stringify!(__alignas_is_defined);
    let s4 = stringify!(__alignof_is_defined);

    if s1!= "_Alignas" {
        std::process::exit(1);
    }
    if s2!= "_Alignof" {
        std::process::exit(1);
    }
    if s3!= "1" {
        std::process::exit(1);
    }
    if s4!= "1" {
        std::process::exit(1);
    }
}