use std::process::exit;

static X1: i32 = 7;
static X2: [u8; 3] = *b"hi\0";

static FMT: [u8; 15] = *b"x1= %d, x2= %s\0";
static INC: [u8; 8] = *b"vers2.h\0";
static A: [u8; 6] = *b"hello\0";
static B: [u8; 13] = *b"hello, world\0";

fn main() {
    if X1 != 7 {
        exit(1);
    }
    if X2[0] != b'h' {
        exit(2);
    }
    if X2[1] != b'i' {
        exit(3);
    }
    if X2[2] != b'\0' {
        exit(4);
    }

    if FMT.len() != 15 {
        exit(5);
    }
    if FMT[0] != b'x' {
        exit(6);
    }
    if FMT[1] != b'1' {
        exit(7);
    }
    if FMT[2] != b'=' {
        exit(8);
    }
    if FMT[3] != b' ' {
        exit(9);
    }
    if FMT[4] != b'%' {
        exit(10);
    }
    if FMT[5] != b'd' {
        exit(11);
    }
    if FMT[6] != b',' {
        exit(12);
    }
    if FMT[7] != b' ' {
        exit(13);
    }
    if FMT[8] != b'x' {
        exit(14);
    }
    if FMT[9] != b'2' {
        exit(15);
    }
    if FMT[10] != b'=' {
        exit(16);
    }
    if FMT[11] != b' ' {
        exit(17);
    }
    if FMT[12] != b'%' {
        exit(18);
    }
    if FMT[13] != b's' {
        exit(19);
    }
    if FMT[14] != b'\0' {
        exit(20);
    }

    if INC.len() != 8 {
        exit(21);
    }
    if INC[0] != b'v' {
        exit(22);
    }
    if INC[1] != b'e' {
        exit(23);
    }
    if INC[2] != b'r' {
        exit(24);
    }
    if INC[3] != b's' {
        exit(25);
    }
    if INC[4] != b'2' {
        exit(26);
    }
    if INC[5] != b'.' {
        exit(27);
    }
    if INC[6] != b'h' {
        exit(28);
    }
    if INC[7] != b'\0' {
        exit(29);
    }

    if A.len() != 6 {
        exit(30);
    }
    if A[0] != b'h' {
        exit(31);
    }
    if A[1] != b'e' {
        exit(32);
    }
    if A[2] != b'l' {
        exit(33);
    }
    if A[3] != b'l' {
        exit(34);
    }
    if A[4] != b'o' {
        exit(35);
    }
    if A[5] != b'\0' {
        exit(36);
    }

    if B.len() != 13 {
        exit(37);
    }
    if B[0] != b'h' {
        exit(38);
    }
    if B[1] != b'e' {
        exit(39);
    }
    if B[2] != b'l' {
        exit(40);
    }
    if B[3] != b'l' {
        exit(41);
    }
    if B[4] != b'o' {
        exit(42);
    }
    if B[5] != b',' {
        exit(43);
    }
    if B[6] != b' ' {
        exit(44);
    }
    if B[7] != b'w' {
        exit(45);
    }
    if B[8] != b'o' {
        exit(46);
    }
    if B[9] != b'r' {
        exit(47);
    }
    if B[10] != b'l' {
        exit(48);
    }
    if B[11] != b'd' {
        exit(49);
    }
    if B[12] != b'\0' {
        exit(50);
    }

    exit(0);
}