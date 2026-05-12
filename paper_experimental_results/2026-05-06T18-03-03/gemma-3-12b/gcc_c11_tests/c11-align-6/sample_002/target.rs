#[cfg(target_pointer_width = "32")]
type signed int = i32;
#[cfg(target_pointer_width = "32")]
type unsigned int = u32;
#[cfg(target_pointer_width = "64")]
type signed int = i64;
#[cfg(target_pointer_width = "64")]
type unsigned int = u64;

#[cfg(target_pointer_width = "32")]
type signed long = i32;
#[cfg(target_pointer_width = "32")]
type unsigned long = u32;
#[cfg(target_pointer_width = "64")]
type signed long = i64;
#[cfg(target_pointer_width = "64")]
type unsigned long = u64;

#[cfg(target_pointer_width = "32")]
type signed long long = i32;
#[cfg(target_pointer_width = "32")]
type unsigned long long = u32;
#[cfg(target_pointer_width = "64")]
type signed long long = i64;
#[cfg(target_pointer_width = "64")]
type unsigned long long = u64;

#[derive(Copy, Clone)]
#[repr(transparent)]
struct _Bool(bool);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct char(u8);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct signed char(i8);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct unsigned char(u8);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct signed short(i16);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct unsigned short(u16);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct float(f32);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct double(f64);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct long double(f64);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct _Complex float([f32; 2]);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct _Complex double([f64; 2]);

#[derive(Copy, Clone)]
#[repr(transparent)]
struct _Complex long double([f64; 2]);

fn main() {
    CHECK_ALIGN(_Bool);
    CHECK_ALIGN(char);
    CHECK_ALIGN(signed char);
    CHECK_ALIGN(unsigned char);
    CHECK_ALIGN(signed short);
    CHECK_ALIGN(unsigned short);
    CHECK_ALIGN(signed int);
    CHECK_ALIGN(unsigned int);
    CHECK_ALIGN(signed long);
    CHECK_ALIGN(unsigned long);
    CHECK_ALIGN(signed long long);
    CHECK_ALIGN(unsigned long long);
    CHECK_ALIGN(float);
    CHECK_ALIGN(double);
    CHECK_ALIGN(long double);
    CHECK_ALIGN(_Complex float);
    CHECK_ALIGN(_Complex double);
    CHECK_ALIGN(_Complex long double);
    std::process::exit(0);
}

macro_rules! CHECK_ALIGN {
    ($TYPE:ty) => {
        struct {
            c: char,
            v: $TYPE,
        } x;
        if std::mem::align_of::<$TYPE>() > std::mem::align_of!(x.v) {
            panic!();
        }
    };
}