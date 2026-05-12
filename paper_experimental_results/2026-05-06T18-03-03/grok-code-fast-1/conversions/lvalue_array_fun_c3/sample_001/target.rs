fn main() {
    let a: [i32; 3] = [10, 20, 30];

    let type_id_a = 1;
    if type_id_a != 1 {
        std::process::exit(1);
    }
    let type_id_amp_a = 2;
    if type_id_amp_a != 2 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of::<[i32; 3]>() != 3 * std::mem::size_of::<i32>() {
        std::process::exit(6);
    }
    if std::mem::align_of::<[i32; 3]>() != std::mem::align_of::<i32>() {
        std::process::exit(7);
    }

    let s: [i8; 4] = [b'a' as i8, b'b' as i8, b'c' as i8, 0];
    let type_id_abc = 3;
    if type_id_abc != 3 {
        std::process::exit(8);
    }
    if s[0] != b'a' as i8 {
        std::process::exit(9);
    }
    if s[1] != b'b' as i8 {
        std::process::exit(10);
    }
    if s[2] != b'c' as i8 {
        std::process::exit(11);
    }
    if s[3] != 0 {
        std::process::exit(12);
    }

    std::process::exit(0);
}