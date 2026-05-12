const X1: i32 = 7;
const X2: &[u8] = b"hi\0";
const FMT: &[u8] = b"x1= %d, x2= %s\0";
const INC: &[u8] = b"vers2.h\0";
const A: &[u8] = b"hello\0";
const B: &[u8] = b"hello, world\0";

fn main() {
    if X1 != 7 {
        std::process::exit(1);
    }
    if X2[0] != b'h' {
        std::process::exit(2);
    }
    if X2[1] != b'i' {
        std::process::exit(3);
    }
    if X2[2] != 0 {
        std::process::exit(4);
    }

    if FMT.len() != 15 {
        std::process::exit(5);
    }
    if FMT[0] != b'x' {
        std::process::exit(6);
    }
    if FMT[1] != b'1' {
        std::process::exit(7);
    }
    if FMT[2] != b'=' {
        std::process::exit(8);
    }
    if FMT[3] != b' ' {
        std::process::exit(9);
    }
    if FMT[4] != b'%' {
        std::process::exit(10);
    }
    if FMT[5] != b'd' {
        std::process::exit(11);
    }
    if FMT[6] != b',' {
        std::process::exit(12);
    }
    if FMT[7] != b' ' {
        std::process::exit(13);
    }
    if FMT[8] != b'x' {
        std::process::exit(14);
    }
    if FMT[9] != b'2' {
        std::process::exit(15);
    }
    if FMT[10] != b'=' {
        std::process::exit(16);
    }
    if FMT[11] != b' ' {
        std::process::exit(17);
    }
    if FMT[12] != b'%' {
        std::process::exit(18);
    }
    if FMT[13] != b's' {
        std::process::exit(19);
    }
    if FMT[14] != 0 {
        std::process::exit(20);
    }

    if INC.len() != 8 {
        std::process::exit(21);
    }
    if INC[0] != b'v' {
        std::process::exit(22);
    }
    if INC[1] != b'e' {
        std::process::exit(23);
    }
    if INC[2] != b'r' {
        std::process::exit(24);
    }
    if INC[3] != b's' {
        std::process::exit(25);
    }
    if INC[4] != b'2' {
        std::process::exit(26);
    }
    if INC[5] != b'.' {
        std::process::exit(27);
    }
    if INC[6] != b'h' {
        std::process::exit(28);
    }
    if INC[7] != 0 {
        std::process::exit(29);
    }

    if A.len() != 6 {
        std::process::exit(30);
    }
    if A[0] != b'h' {
        std::process::exit(31);
    }
    if A[1] != b'e' {
        std::process::exit(32);
    }
    if A[2] != b'l' {
        std::process::exit(33);
    }
    if A[3] != b'l' {
        std::process::exit(34);
    }
    if A[4] != b'o' {
        std::process::exit(35);
    }
    if A[5] != 0 {
        std::process::exit(36);
    }

    if B.len() != 13 {
        std::process::exit(37);
    }
    if B[0] != b'h' {
        std::process::exit(38);
    }
    if B[1] != b'e' {
        std::process::exit(39);
    }
    if B[2] != b'l' {
        std::process::exit(40);
    }
    if B[3] != b'l' {
        std::process::exit(41);
    }
    if B[4] != b'o' {
        std::process::exit(42);
    }
    if B[5] != b',' {
        std::process::exit(43);
    }
    if B[6] != b' ' {
        std::process::exit(44);
    }
    if B[7] != b'w' {
        std::process::exit(45);
    }
    if B[8] != b'o' {
        std::process::exit(46);
    }
    if B[9] != b'r' {
        std::process::exit(47);
    }
    if B[10] != b'l' {
        std::process::exit(48);
    }
    if B[11] != b'd' {
        std::process::exit(49);
    }
    if B[12] != 0 {
        std::process::exit(50);
    }

    std::process::exit(0);
}