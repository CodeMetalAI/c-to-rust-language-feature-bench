const X1: i32 = 7;
const X2: &[u8] = b"hi\x00";
const FMT: &[u8] = b"x1= %d, x2= %s\x00";
const INC: &[u8] = b"vers2.h\x00";
const A: &[u8] = b"hello\x00";
const B: &[u8] = b"hello, world\x00";

fn main() {
    use std::process;

    if X1 != 7 {
        process::exit(1);
    }
    if X2[0] != b'h' {
        process::exit(2);
    }
    if X2[1] != b'i' {
        process::exit(3);
    }
    if X2[2] != b'\0' {
        process::exit(4);
    }

    if FMT.len() != 15 {
        process::exit(5);
    }
    if FMT[0] != b'x' {
        process::exit(6);
    }
    if FMT[1] != b'1' {
        process::exit(7);
    }
    if FMT[2] != b'=' {
        process::exit(8);
    }
    if FMT[3] != b' ' {
        process::exit(9);
    }
    if FMT[4] != b'%' {
        process::exit(10);
    }
    if FMT[5] != b'd' {
        process::exit(11);
    }
    if FMT[6] != b',' {
        process::exit(12);
    }
    if FMT[7] != b' ' {
        process::exit(13);
    }
    if FMT[8] != b'x' {
        process::exit(14);
    }
    if FMT[9] != b'2' {
        process::exit(15);
    }
    if FMT[10] != b'=' {
        process::exit(16);
    }
    if FMT[11] != b' ' {
        process::exit(17);
    }
    if FMT[12] != b'%' {
        process::exit(18);
    }
    if FMT[13] != b's' {
        process::exit(19);
    }
    if FMT[14] != b'\0' {
        process::exit(20);
    }

    if INC.len() != 8 {
        process::exit(21);
    }
    if INC[0] != b'v' {
        process::exit(22);
    }
    if INC[1] != b'e' {
        process::exit(23);
    }
    if INC[2] != b'r' {
        process::exit(24);
    }
    if INC[3] != b's' {
        process::exit(25);
    }
    if INC[4] != b'2' {
        process::exit(26);
    }
    if INC[5] != b'.' {
        process::exit(27);
    }
    if INC[6] != b'h' {
        process::exit(28);
    }
    if INC[7] != b'\0' {
        process::exit(29);
    }

    if A.len() != 6 {
        process::exit(30);
    }
    if A[0] != b'h' {
        process::exit(31);
    }
    if A[1] != b'e' {
        process::exit(32);
    }
    if A[2] != b'l' {
        process::exit(33);
    }
    if A[3] != b'l' {
        process::exit(34);
    }
    if A[4] != b'o' {
        process::exit(35);
    }
    if A[5] != b'\0' {
        process::exit(36);
    }

    if B.len() != 13 {
        process::exit(37);
    }
    if B[0] != b'h' {
        process::exit(38);
    }
    if B[1] != b'e' {
        process::exit(39);
    }
    if B[2] != b'l' {
        process::exit(40);
    }
    if B[3] != b'l' {
        process::exit(41);
    }
    if B[4] != b'o' {
        process::exit(42);
    }
    if B[5] != b',' {
        process::exit(43);
    }
    if B[6] != b' ' {
        process::exit(44);
    }
    if B[7] != b'w' {
        process::exit(45);
    }
    if B[8] != b'o' {
        process::exit(46);
    }
    if B[9] != b'r' {
        process::exit(47);
    }
    if B[10] != b'l' {
        process::exit(48);
    }
    if B[11] != b'd' {
        process::exit(49);
    }
    if B[12] != b'\0' {
        process::exit(50);
    }

    process::exit(0);
}