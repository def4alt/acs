use std::{any::type_name, time::Instant};

mod tests;

use num_traits::{Bounded, Num};
use tests::*;

struct Test {
    pub op: char,
    pub typ: &'static str,
    pub ops_pro_ns: f64,
}

macro_rules! call_for_each_type {
    ($func: ident $(, $t: ty)+) => {
        {
            let mut tests: Vec<Test> = Vec::new();

            $(tests.append(&mut $func::<$t>());)+

            tests
        }
    };
}

fn run_basic_ops<T: Num + Copy + Bounded>() -> Vec<Test> {
    let mut tests = Vec::new();
    let t_name = type_name::<T>();

    for op in "+-/*".chars() {
        let test = match op {
            '+' => Test {
                op,
                typ: t_name,
                ops_pro_ns: get_ops_pro_ns(add::<T>),
            },
            '-' => Test {
                op,
                typ: t_name,
                ops_pro_ns: get_ops_pro_ns(substract::<T>),
            },
            '/' => Test {
                op,
                typ: t_name,
                ops_pro_ns: get_ops_pro_ns(divide::<T>),
            },
            '*' => Test {
                op,
                typ: t_name,
                ops_pro_ns: get_ops_pro_ns(multiply::<T>),
            },
            _ => panic!(),
        };
        tests.push(test);
    }

    tests
}

fn main() {
    let mut tests: Vec<Test> =
        call_for_each_type!(run_basic_ops, i128, i64, i32, i16, i8, f64, f32);

    let max = tests.iter().map(|t| t.ops_pro_ns).reduce(f64::max).unwrap();

    tests.sort_by_key(|k| k.op);

    for test in tests.iter() {
        println!(
            "{:<5} {:<5} {:>10.3} * 10^7 \t {:<40} {:.0}%",
            test.op,
            test.typ,
            test.ops_pro_ns * 100.0,
            "X".repeat((test.ops_pro_ns * 35.0 / max) as usize),
            test.ops_pro_ns * 100.0 / max
        );
    }
}

fn get_ops_pro_ns<F>(func: F) -> f64
where
    F: Fn(),
{
    let mut sum = 0;

    for _ in 1..10 {
        sum += get_function_duration(&func);
    }

    let average = sum as f64 / 10.0;
    10000.0 / (average)
}

fn get_function_duration<F>(func: F) -> u64
where
    F: Fn(),
{
    let now = Instant::now();
    func();
    now.elapsed().as_nanos() as u64
}
