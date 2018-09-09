//extern crate cfg_if;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// mod utils;

enum CalcOperator {
   Add,
   Subtract,
   Divide,
   Multiply,
}

struct Calculation {
    current_value: f32,
    new_input: f32,
    operator: CalcOperator,
}

impl Calculation {
    fn new(a: f32, b: f32, operator: char) -> Calculation {
        Calculation {
            current_value: a,
            new_input: b,
            operator: determine_operator(operator),
        }
    }
}

#[wasm_bindgen]
pub fn calculate(a: f32, b:f32, operator: char) -> f32 {
    let a_calc = Calculation::new(a, b, operator);
    perform_calculation(a_calc)
}

fn determine_operator(operator:char) -> CalcOperator {
    match operator {
        '+' => CalcOperator::Add,
        '-' => CalcOperator::Subtract,
        '/' => CalcOperator::Divide,
        '*' => CalcOperator::Multiply,
        _ => panic!("Invalid operator")
    }
}

fn perform_calculation(calculation: Calculation) -> f32 {
    let operator = calculation.operator;
    match operator {
        CalcOperator::Multiply => multiply(calculation.current_value, calculation.new_input),
        CalcOperator::Divide => divide(calculation.current_value, calculation.new_input),
        CalcOperator::Add => add(calculation.current_value, calculation.new_input),
        CalcOperator::Subtract => subtract(calculation.current_value, calculation.new_input),
    }
}

fn subtract(a:f32, b:f32) -> f32 {
    a - b
}

fn add(a:f32, b:f32) -> f32 {
    a + b
}

fn multiply(a:f32, b:f32) -> f32 {
    a * b
}

fn divide(numerator:f32, denominator:f32) -> f32 {
    numerator / denominator
}
