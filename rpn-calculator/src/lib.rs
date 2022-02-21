#[derive(Debug, PartialEq)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut values: Vec<i32> = vec![];
    for input in inputs.iter() {
        match input {
            CalculatorInput::Value(value) => values.push(*value),
            _ if values.len() < 2 => return None,
            CalculatorInput::Add => {
                let (x, y) = get_last_two_values_in_order(&mut values);
                values.push(x + y);
            }
            CalculatorInput::Subtract => {
                let (x, y) = get_last_two_values_in_order(&mut values);
                values.push(x - y);
            }
            CalculatorInput::Multiply => {
                let (x, y) = get_last_two_values_in_order(&mut values);
                values.push(x * y);
            }
            CalculatorInput::Divide => {
                let (x, y) = get_last_two_values_in_order(&mut values);
                values.push(x / y);
            }
        }
    }

    if values.len() == 1 {
        Some(values[0])
    } else {
        None
    }
}

fn get_last_two_values_in_order(values: &mut Vec<i32>) -> (i32, i32) {
    (
        values.remove(values.len() - 2),
        values.remove(values.len() - 1),
    )
}
