use std::ops::{Add, Div, Mul, Sub};

use num_traits::{one, Bounded, Num};

macro_rules! x10 {
    ($function:expr) => {
        $function();
        $function();
        $function();
        $function();
        $function();

        $function();
        $function();
        $function();
        $function();
        $function();
    };
}

macro_rules! x100 {
    ($function:expr) => {
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

pub fn body<F>(func: F)
where
    F: Fn(),
{
    for _ in 1..1000 {
        func();
    }
}

pub fn mock() {
    body(|| {
        std::hint::black_box(0);
    });
}

pub fn add<T: Num + Add<Output = T> + Copy + Div<Output = T> + Bounded>() {
    let max = <T>::max_value() / (one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>());
    let value = one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>();
    body(|| {
        x100!(|| { std::hint::black_box(max + value) });
    })
}

pub fn substract<T: Num + Add<Output = T> + Copy + Div<Output = T> + Sub<Output = T> + Bounded>() {
    let max = <T>::max_value();
    let value = one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>();
    body(|| {
        x100!(|| { std::hint::black_box(max - value) });
    })
}

pub fn divide<T: Num + Add<Output = T> + Copy + Div<Output = T> + Bounded>() {
    let max = <T>::max_value();
    let value = one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>();
    body(|| {
        x100!(|| { std::hint::black_box(max / value) });
    })
}

pub fn multiply<T: Num + Add<Output = T> + Copy + Div<Output = T> + Mul<Output = T> + Bounded>() {
    let max = <T>::max_value() / (one::<T>() + one::<T>() + one::<T>() + one::<T>() + one::<T>());
    let value = one::<T>() + one::<T>();
    body(|| {
        x100!(|| { std::hint::black_box(max * value) });
    })
}
