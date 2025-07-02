use std::env::{args, Args};

fn main() {
// let args: Args = args();
// println!("{:?}", args)

  let mut args: Args = args();
  let first: String = args.nth(1).unwrap();

  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second: String = args.nth(0).unwrap();
  let first_number: f32 = first.parse::<f32>().unwrap();
  let second_number: f32 = second.parse::<f32>().unwrap();

  let result = operate(operator, first_number, second_number);
  

  println!("{} {} {} = {}", first_number, operator, second_number, result); // returns an f32

  println!("{}", output(operator, first_number, second_number, result)); // returns a string
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '*' | 'x' | 'X' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}

fn output(operator: char, first_number: f32, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}