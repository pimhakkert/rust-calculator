use clap::Parser;
use std::io;
use regex::Regex;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    println!("Welcome to the CALCULATOR! Enter your input or type 'exit' to quit.");
    println!("Make sure calculations are written as: '10 + 5' or any variation of that.");
    println!("Use 'ans' in a calculation to use the answer of the previous calculation, or type 'clear' to clear that number.");

    let allowed_operators: [char; 5] = ['+', '-', '*', '/', '%'];

    // The current number to base a calculation off of
    let mut buffered_num: f32 = 0.0;

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line!");

        input = input.trim().parse().unwrap();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if input.eq_ignore_ascii_case("clear") {
            println!("Cleared previous answer!");
            buffered_num = 0.0;
            continue;
        }

        //Validate input to not have invalid characters
        let regex = Regex::new(r"^\s*((\d+(\.\d+)?|ans)\s*[\+\-\*\/%]\s*)*(\d+(\.\d+)?|ans)\s*$").unwrap();

        if regex.is_match(&*input) == false {
            println!("Invalid input!");
            break;
        }

        //Example: "10 + 5"
        let input_arr: Vec<&str> = input.split_whitespace().collect();

        let mut operators: Vec<char> = Vec::new();
        let mut numbers: Vec<f32> = Vec::new();

        for substring in input_arr {
            match substring.parse::<f32>() {
                Ok(num) => {
                    numbers.push(num);
                },
                Err(_) => {
                    if substring == "ans" {
                        //Take the result from the previous question, or just 0
                        numbers.push(buffered_num);
                    } else {
                        //This assumes that operator strings are chars, and the regex SHOULD cover that.
                        operators.push(substring.parse().unwrap());
                    }
                }
            }
        }

        if operators.len() > 1 {
            println!("Current version does not support multi-operator calculations.");
            break;
        }

        //Version 1.0 does not do the following:
        //1. work with multiple operators
        //3. take operator precedence into account
        //4. recognize parenthesis

        //TODO make this logic work with more than just 1 operation
        let result: f32 = apply_operator(operators[0], numbers[0], numbers[1]);
        println!("{}", result);

        buffered_num = result;
    }
}

fn apply_operator(operator: char, left: f32, right: f32) -> f32 {
    match operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        '%' => left % right,
        _ => panic!("Invalid operator!")
    }
}