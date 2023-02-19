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

pub fn add_i128() {
    body(|| {
        std::hint::black_box(18i128 + 540i128);
    })
}

pub fn add_i64() {
    body(|| {
        std::hint::black_box(65i64 + 76i64);
    })
}
pub fn add_i32() {
    body(|| {
        std::hint::black_box(125i32 + 65i32);
    })
}
pub fn add_i16() {
    body(|| {
        std::hint::black_box(12i16 + 3i16);
    })
}
pub fn add_i8() {
    body(|| {
        std::hint::black_box(3i8 + 4i8);
    })
}
pub fn add_f64() {
    body(|| {
        std::hint::black_box(10546456.34525f64 + 75675.234f64);
    })
}
pub fn add_f32() {
    body(|| {
        std::hint::black_box(12665.324f32 + 243.565f32);
    })
}
pub fn substract_i128() {
    body(|| {
        std::hint::black_box(std::i128::MAX - 234235i128);
    })
}
pub fn substract_i64() {
    body(|| {
        std::hint::black_box(std::i64::MAX - 234235i64);
    })
}
pub fn substract_i32() {
    body(|| {
        std::hint::black_box(std::i32::MAX - 24234i32);
    })
}
pub fn substract_i16() {
    body(|| {
        std::hint::black_box(std::i16::MAX - 23423i16);
    })
}
pub fn substract_i8() {
    body(|| {
        std::hint::black_box(std::i8::MAX - 23i8);
    })
}
pub fn substract_f64() {
    body(|| {
        std::hint::black_box(std::f64::MAX - 53254.7657f64);
    })
}
pub fn substract_f32() {
    body(|| {
        std::hint::black_box(std::f32::MAX - 3236.543f32);
    })
}

pub fn divide_i128() {
    body(|| {
        std::hint::black_box(std::i128::MAX / 1245i128);
    })
}
pub fn divide_i64() {
    body(|| {
        std::hint::black_box(std::i64::MAX / 6545i64);
    })
}
pub fn divide_i32() {
    body(|| {
        std::hint::black_box(std::i32::MAX / 4234i32);
    })
}
pub fn divide_i16() {
    body(|| {
        std::hint::black_box(std::i16::MAX / 346i16);
    })
}
pub fn divide_i8() {
    body(|| {
        std::hint::black_box(std::i8::MAX / 12i8);
    })
}
pub fn divide_f64() {
    body(|| {
        std::hint::black_box(std::f64::MAX / 165.643f64);
    })
}
pub fn divide_f32() {
    body(|| {
        std::hint::black_box(std::f32::MAX / 347.1235f32);
    })
}

pub fn multiply_i128() {
    body(|| {
        std::hint::black_box(234i128 * 354754i128);
    })
}

pub fn multiply_i64() {
    body(|| {
        std::hint::black_box(653i64 * 234i64);
    })
}
pub fn multiply_i32() {
    body(|| {
        std::hint::black_box(658i32 * 2346i32);
    })
}
pub fn multiply_i16() {
    body(|| {
        std::hint::black_box(12i16 * 7i16);
    })
}
pub fn multiply_i8() {
    body(|| {
        std::hint::black_box(3i8 * 5i8);
    })
}
pub fn multiply_f64() {
    body(|| {
        std::hint::black_box(236457.235f64 * 75675.234f64);
    })
}
pub fn multiply_f32() {
    body(|| {
        std::hint::black_box(4745.234f32 * 75685.234f32);
    })
}
