fn main() {
    let x1 = 7;
    let x2 = "hi";

    let fmt = "x1= %d, x2= %s";
    let inc = "vers2.h";
    let a = "hello";
    let b = "hello, world";

    if x1 != 7 {
        std::process::exit(1);
    }
    let x2_bytes = x2.as_bytes();
    if x2_bytes[0] != b'h' {
        std::process::exit(2);
    }
    if x2_bytes[1] != b'i' {
        std::process::exit(3);
    }
    if x2_bytes.len() != 2 {
        std::process::exit(4);
    }

    if fmt.len() != 14 {
        std::process::exit(5);
    }
    let fmt_bytes = fmt.as_bytes();
    if fmt_bytes[0] != b'x' {
        std::process::exit(6);
    }
    if fmt_bytes[1] != b'1' {
        std::process::exit(7);
    }
    if fmt_bytes[2] != b'=' {
        std::process::exit(8);
    }
    if fmt_bytes[3] != b' ' {
        std::process::exit(9);
    }
    if fmt_bytes[4] != b'%' {
        std::process::exit(10);
    }
    if fmt_bytes[5] != b'd' {
        std::process::exit(11);
    }
    if fmt_bytes[6] != b',' {
        std::process::exit(12);
    }
    if fmt_bytes[7] != b' ' {
        std::process::exit(13);
    }
    if fmt_bytes[8] != b'x' {
        std::process::exit(14);
    }
    if fmt_bytes[9] != b'2' {
        std::process::exit(15);
    }
    if fmt_bytes[10] != b'=' {
        std::process::exit(16);
    }
    if fmt_bytes[11] != b' ' {
        std::process::exit(17);
    }
    if fmt_bytes[12] != b'%' {
        std::process::exit(18);
    }
    if fmt_bytes[13] != b's' {
        std::process::exit(19);
    }

    if inc.len() != 7 {
        std::process::exit(21);
    }
    let inc_bytes = inc.as_bytes();
    if inc_bytes[0] != b'v' {
        std::process::exit(22);
    }
    if inc_bytes[1] != b'e' {
        std::process::exit(23);
    }
    if inc_bytes[2] != b'r' {
        std::process::exit(24);
    }
    if inc_bytes[3] != b's' {
        std::process::exit(25);
    }
    if inc_bytes[4] != b'2' {
        std::process::exit(26);
    }
    if inc_bytes[5] != b'.' {
        std::process::exit(27);
    }
    if inc_bytes[6] != b'h' {
        std::process::exit(28);
    }

    if a.len() != 5 {
        std::process::exit(30);
    }
    let a_bytes = a.as_bytes();
    if a_bytes[0] != b'h' {
        std::process::exit(31);
    }
    if a_bytes[1] != b'e' {
        std::process::exit(32);
    }
    if a_bytes[2] != b'l' {
        std::process::exit(33);
    }
    if a_bytes[3] != b'l' {
        std::process::exit(34);
    }
    if a_bytes[4] != b'o' {
        std::process::exit(35);
    }

    if b.len() != 12 {
        std::process::exit(37);
    }
    let b_bytes = b.as_bytes();
    if b_bytes[0] != b'h' {
        std::process::exit(38);
    }
    if b_bytes[1] != b'e' {
        std::process::exit(39);
    }
    if b_bytes[2] != b'l' {
        std::process::exit(40);
    }
    if b_bytes[3] != b'l' {
        std::process::exit(41);
    }
    if b_bytes[4] != b'o' {
        std::process::exit(42);
    }
    if b_bytes[5] != b',' {
        std::process::exit(43);
    }
    if b_bytes[6] != b' ' {
        std::process::exit(44);
    }
    if b_bytes[7] != b'w' {
        std::process::exit(45);
    }
    if b_bytes[8] != b'o' {
        std::process::exit(46);
    }
    if b_bytes[9] != b'r' {
        std::process::exit(47);
    }
    if b_bytes[10] != b'l' {
        std::process::exit(48);
    }
    if b_bytes[11] != b'd' {
        std::process::exit(49);
    }
}