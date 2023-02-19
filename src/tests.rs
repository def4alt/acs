use std::ops::{Add, Div, Mul, Sub};

use num_traits::{one, Bounded, Num};

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

pub fn add<T: Num + Add<Output = T> + Copy + Div<Output = T> + Bounded>() {
    let max = <T>::max_value() / (one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>());
    let value = one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>();
    body(|| {
        std::hint::black_box(max + value);
    })
}

pub fn substract<T: Num + Add<Output = T> + Copy + Div<Output = T> + Sub<Output = T> + Bounded>() {
    let max = <T>::max_value();
    let value = one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>();
    body(|| {
        std::hint::black_box(max - value);
    })
}

pub fn divide<T: Num + Add<Output = T> + Copy + Div<Output = T> + Bounded>() {
    let max = <T>::max_value();
    let value = one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>();
    body(|| {
        std::hint::black_box(max / value);
    })
}

pub fn multiply<T: Num + Add<Output = T> + Copy + Div<Output = T> + Mul<Output = T> + Bounded>() {
    let max = <T>::max_value() / (one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>());
    let value = one::<T>() + one::<T>();
    body(|| {
        std::hint::black_box(max * value);
    })
}
