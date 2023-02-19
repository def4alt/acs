macro_rules! x10 {
    ($function:ident) => {
        std::hint::black_box($function());
        std::hint::black_box($function());
        std::hint::black_box($function());
        std::hint::black_box($function());
        std::hint::black_box($function());

        std::hint::black_box($function());
        std::hint::black_box($function());
        std::hint::black_box($function());
        std::hint::black_box($function());
        std::hint::black_box($function());
    };
}

macro_rules! x100 {
    ($function:ident) => {
        x10!($function);
        x10!($function);
        x10!($function);
        x10!($function);
        x10!($function);

        x10!($function);
        x10!($function);
        x10!($function);
        x10!($function);
        x10!($function);
    };
}

macro_rules! x1000 {
    ($function:ident) => {
        x100!($function);
        x100!($function);
        x100!($function);
        x100!($function);
        x100!($function);

        x100!($function);
        x100!($function);
        x100!($function);
        x100!($function);
        x100!($function);
    };
}

macro_rules! x10000 {
    ($function:ident) => {
        x1000!($function);
        x1000!($function);
        x1000!($function);
        x1000!($function);
        x1000!($function);

        x1000!($function);
        x1000!($function);
        x1000!($function);
        x1000!($function);
        x1000!($function);
    };
}

pub fn body<F>(func: F)
where
    F: Fn(),
{
    x10000!(func);
}

const NUM_I128: i128 = std::i128::MAX / 8;
const NUM_I64: i64 = std::i64::MAX / 9;
const NUM_I32: i32 = std::i32::MAX / 7;
const NUM_I16: i16 = std::i16::MAX / 3;
const NUM_I8: i8 = std::i8::MAX / 5;
const NUM_F64: f64 = std::f64::MAX / 654.43563;
const NUM_F32: f32 = std::f32::MAX / 764.876867;

pub fn add_i128() {
    body(|| {
        std::hint::black_box(NUM_I128 + 40);
    })
}

pub fn add_i64() {
    body(|| {
        std::hint::black_box(NUM_I64 + 76);
    })
}

pub fn add_i32() {
    body(|| {
        std::hint::black_box(NUM_I32 + 65);
    })
}
pub fn add_i16() {
    body(|| {
        std::hint::black_box(NUM_I16 + 3);
    })
}
pub fn add_i8() {
    body(|| {
        std::hint::black_box(NUM_I8 + 3);
    })
}
pub fn add_f64() {
    body(|| {
        std::hint::black_box(NUM_F64 + 75.234);
    })
}
pub fn add_f32() {
    body(|| {
        std::hint::black_box(NUM_F32 + 243.565);
    })
}
pub fn substract_i128() {
    body(|| {
        std::hint::black_box(std::i128::MAX - 234235);
    })
}
pub fn substract_i64() {
    body(|| {
        std::hint::black_box(std::i64::MAX - 234235);
    })
}
pub fn substract_i32() {
    body(|| {
        std::hint::black_box(std::i32::MAX - 24234);
    })
}
pub fn substract_i16() {
    body(|| {
        std::hint::black_box(std::i16::MAX - 23423);
    })
}
pub fn substract_i8() {
    body(|| {
        std::hint::black_box(std::i8::MAX - 23);
    })
}
pub fn substract_f64() {
    body(|| {
        std::hint::black_box(std::f64::MAX - 53254.7657);
    })
}
pub fn substract_f32() {
    body(|| {
        std::hint::black_box(std::f32::MAX - 3236.543);
    })
}

pub fn divide_i128() {
    body(|| {
        std::hint::black_box(std::i128::MAX / 1245);
    })
}
pub fn divide_i64() {
    body(|| {
        std::hint::black_box(std::i64::MAX / 6545);
    })
}
pub fn divide_i32() {
    body(|| {
        std::hint::black_box(std::i32::MAX / 4234);
    })
}
pub fn divide_i16() {
    body(|| {
        std::hint::black_box(std::i16::MAX / 346);
    })
}
pub fn divide_i8() {
    body(|| {
        std::hint::black_box(std::i8::MAX / 12);
    })
}
pub fn divide_f64() {
    body(|| {
        std::hint::black_box(std::f64::MAX / 165.643);
    })
}
pub fn divide_f32() {
    body(|| {
        std::hint::black_box(std::f32::MAX / 347.1235);
    })
}

pub fn multiply_i128() {
    body(|| {
        std::hint::black_box(NUM_I128 * 6);
    })
}

pub fn multiply_i64() {
    body(|| {
        std::hint::black_box(NUM_I64 * 2);
    })
}
pub fn multiply_i32() {
    body(|| {
        std::hint::black_box(NUM_I32 * 3);
    })
}
pub fn multiply_i16() {
    body(|| {
        std::hint::black_box(NUM_I16 * 2);
    })
}
pub fn multiply_i8() {
    body(|| {
        std::hint::black_box(NUM_I8 * 2);
    })
}
pub fn multiply_f64() {
    body(|| {
        std::hint::black_box(NUM_F64 * 7.234f64);
    })
}
pub fn multiply_f32() {
    body(|| {
        std::hint::black_box(NUM_F32 * 5.234f32);
    })
}
