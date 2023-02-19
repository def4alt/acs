use std::{time::Instant, any::type_name, collections::HashMap};

mod tests;

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

fn run_basic_ops<T>() -> Vec<Test> {
    let mut tests = Vec::new();
    let t_name = type_name::<T>();
    let operations = vec!["add", "substract", "divide", "multiply"];
    let function_map: HashMap<&str, fn()> = HashMap::from([
        ("add_i128", add_i128 as fn()),
        ("add_i64", add_i64 as fn()),
        ("add_i32", add_i32 as fn()),
        ("add_i16", add_i16 as fn()),
        ("add_i8", add_i8 as fn()),
        ("add_f64", add_f64 as fn()),
        ("add_f32", add_f32 as fn()),
        ("substract_i128", substract_i128 as fn()),
        ("substract_i64", substract_i64 as fn()),
        ("substract_i32", substract_i32 as fn()),
        ("substract_i16", substract_i16 as fn()),
        ("substract_i8", substract_i8 as fn()),
        ("substract_f64", substract_f64 as fn()),
        ("substract_f32", substract_f32 as fn()),
        ("divide_i128", divide_i128 as fn()),
        ("divide_i64", divide_i64 as fn()),
        ("divide_i32", divide_i32 as fn()),
        ("divide_i16", divide_i16 as fn()),
        ("divide_i8", divide_i8 as fn()),
        ("divide_f64", divide_f64 as fn()),
        ("divide_f32", divide_f32 as fn()),
        ("multiply_i128", multiply_i128 as fn()),
        ("multiply_i64", multiply_i64 as fn()),
        ("multiply_i32", multiply_i32 as fn()),
        ("multiply_i16", multiply_i16 as fn()),
        ("multiply_i8", multiply_i8 as fn()),
        ("multiply_f64", multiply_f64 as fn()),
        ("multiply_f32", multiply_f32 as fn()),
    ]);

    for op in operations {
        let function_name: String = op.to_owned() + "_" + &t_name.to_owned();
        let function = function_map.get(function_name.as_str()).unwrap();
        let ops_pro_ns = get_ops_pro_ns(function);
        let op_char = match op {
            "add" => '+',
            "substract" => '-',
            "divide" => '/',
            "multiply" => '*',
            _ => ' '
        }; 
        tests.push(Test { op: op_char, typ: t_name, ops_pro_ns });
    }

    tests
}

fn main() {
    let mut tests: Vec<Test> = call_for_each_type!(run_basic_ops, i128, i64, i32, i16, i8, f64, f32);

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

