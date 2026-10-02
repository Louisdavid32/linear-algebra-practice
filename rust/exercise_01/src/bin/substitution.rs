
use std::io::{self, Write};

const EPSILON: f64 = 1e-10;

fn read_number(name: &str) -> f64 {
    loop {
        print!("{} = ", name);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Please enter a valid number.");
            }
        }
    }
}

fn main() {
    println!("Exercise 01 — Simple 2x2 Linear System");
    println!();
    println!("We solve a system of the form:");
    println!("a1*x + b1*y = c1");
    println!("a2*x + b2*y = c2");
    println!();

    // First equation
    let a1 = read_number("a1");
    let b1 = read_number("b1");
    let c1 = read_number("c1");

    println!();

    // Second equation
    let a2 = read_number("a2");
    let b2 = read_number("b2");
    let c2 = read_number("c2");

    println!();
    println!("System:");
    println!("{a1}x + ({b1})y = {c1}");
    println!("{a2}x + ({b2})y = {c2}");

    println!();
    println!("--- Substitution method ---");

    // From:
    //
    // a1*x + b1*y = c1
    //
    // isolate y:
    //
    // y = (c1 - a1*x) / b1

    if b1.abs() < EPSILON {
        println!("Cannot isolate y from the first equation.");
        return;
    }

    println!();
    println!("From equation 1:");
    println!("y = ({c1} - {a1}x) / {b1}");

    // Substitute into equation 2:
    //
    // a2*x + b2*((c1 - a1*x)/b1) = c2

    let denominator = a2 - (b2 * a1 / b1);
    let numerator = c2 - (b2 * c1 / b1);

    if denominator.abs() < EPSILON {
        if numerator.abs() < EPSILON {
            println!();
            println!("The system has infinitely many solutions.");
        } else {
            println!();
            println!("The system has no solution.");
        }

        return;
    }

    let x = numerator / denominator;

    // Substitute x back into equation 1
    let y = (c1 - a1 * x) / b1;

    println!();
    println!("After substitution:");
    println!("x = {x}");

    println!();
    println!("Substitute x back into equation 1:");
    println!("y = ({c1} - {a1} * {x}) / {b1}");
    println!("y = {y}");

    println!();
    println!("--- Solution ---");
    println!("x = {x}");
    println!("y = {y}");

    // Verification
    let equation_1 = a1 * x + b1 * y;
    let equation_2 = a2 * x + b2 * y;

    println!();
    println!("--- Verification ---");
    println!("Equation 1: {equation_1} = {c1}");
    println!("Equation 2: {equation_2} = {c2}");
}
