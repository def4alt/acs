use std::time::Instant;

mod tests;

use tests::*;

struct Test {
    pub op: char,
    pub typ: &'static str,
    pub ops_pro_ns: f64,
}


fn main() {
    let mut tests: Vec<Test> = Vec::new();

    for op in "+-/*".chars() {
        match op {
            '+' => {
                tests.push(Test {
                    op,
                    typ: "i128",
                    ops_pro_ns: get_ops_pro_ns(add_i128),
                });
                tests.push(Test {
                    op,
                    typ: "i64",
                    ops_pro_ns: get_ops_pro_ns(add_i64),
                });
                tests.push(Test {
                    op,
                    typ: "i32",
                    ops_pro_ns: get_ops_pro_ns(add_i32),
                });
                tests.push(Test {
                    op,
                    typ: "i16",
                    ops_pro_ns: get_ops_pro_ns(add_i16),
                });
                tests.push(Test {
                    op,
                    typ: "i8",
                    ops_pro_ns: get_ops_pro_ns(add_i8),
                });
                tests.push(Test {
                    op,
                    typ: "f64",
                    ops_pro_ns: get_ops_pro_ns(add_f64),
                });
                tests.push(Test {
                    op,
                    typ: "f32",
                    ops_pro_ns: get_ops_pro_ns(add_f32),
                });
            }
            '-' => {
                tests.push(Test {
                    op,
                    typ: "i128",
                    ops_pro_ns: get_ops_pro_ns(substract_i128),
                });
                tests.push(Test {
                    op,
                    typ: "i64",
                    ops_pro_ns: get_ops_pro_ns(substract_i64),
                });
                tests.push(Test {
                    op,
                    typ: "i32",
                    ops_pro_ns: get_ops_pro_ns(substract_i32),
                });
                tests.push(Test {
                    op,
                    typ: "i16",
                    ops_pro_ns: get_ops_pro_ns(substract_i16),
                });
                tests.push(Test {
                    op,
                    typ: "i8",
                    ops_pro_ns: get_ops_pro_ns(substract_i8),
                });
                tests.push(Test {
                    op,
                    typ: "f64",
                    ops_pro_ns: get_ops_pro_ns(substract_f64),
                });
                tests.push(Test {
                    op,
                    typ: "f32",
                    ops_pro_ns: get_ops_pro_ns(substract_f32),
                });
            }
            '/' => {
                tests.push(Test {
                    op,
                    typ: "i128",
                    ops_pro_ns: get_ops_pro_ns(divide_i128),
                });
                tests.push(Test {
                    op,
                    typ: "i64",
                    ops_pro_ns: get_ops_pro_ns(divide_i64),
                });
                tests.push(Test {
                    op,
                    typ: "i32",
                    ops_pro_ns: get_ops_pro_ns(divide_i32),
                });
                tests.push(Test {
                    op,
                    typ: "i16",
                    ops_pro_ns: get_ops_pro_ns(divide_i16),
                });
                tests.push(Test {
                    op,
                    typ: "i8",
                    ops_pro_ns: get_ops_pro_ns(divide_i8),
                });
                tests.push(Test {
                    op,
                    typ: "f64",
                    ops_pro_ns: get_ops_pro_ns(divide_f64),
                });
                tests.push(Test {
                    op,
                    typ: "f32",
                    ops_pro_ns: get_ops_pro_ns(divide_f32),
                });
            }
            '*' => {
                tests.push(Test {
                    op,
                    typ: "i128",
                    ops_pro_ns: get_ops_pro_ns(multiply_i128),
                });
                tests.push(Test {
                    op,
                    typ: "i64",
                    ops_pro_ns: get_ops_pro_ns(multiply_i64),
                });
                tests.push(Test {
                    op,
                    typ: "i32",
                    ops_pro_ns: get_ops_pro_ns(multiply_i32),
                });
                tests.push(Test {
                    op,
                    typ: "i16",
                    ops_pro_ns: get_ops_pro_ns(multiply_i16),
                });
                tests.push(Test {
                    op,
                    typ: "i8",
                    ops_pro_ns: get_ops_pro_ns(multiply_i8),
                });
                tests.push(Test {
                    op,
                    typ: "f64",
                    ops_pro_ns: get_ops_pro_ns(multiply_f64),
                });
                tests.push(Test {
                    op,
                    typ: "f32",
                    ops_pro_ns: get_ops_pro_ns(multiply_f32),
                });
            }
            _ => (),
        }
    }

    let max = tests.iter().map(|t| t.ops_pro_ns).reduce(f64::max).unwrap();

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

