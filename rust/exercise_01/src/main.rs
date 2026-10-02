use nalgebra::{Matrix2, Vector2};
use std::io::{self, Write};

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
    println!("Exercise 01 — Matrix solution with nalgebra");
    println!();

    println!("System:");
    println!("a1*x + b1*y = c1");
    println!("a2*x + b2*y = c2");
    println!();

    let a1 = read_number("a1");
    let b1 = read_number("b1");
    let c1 = read_number("c1");

    println!();

    let a2 = read_number("a2");
    let b2 = read_number("b2");
    let c2 = read_number("c2");

    // Coefficient matrix A
    let a = Matrix2::new(
        a1, b1,
        a2, b2,
    );

    // Vector b
    let b = Vector2::new(
        c1,
        c2,
    );

    println!();
    println!("Matrix A:");
    println!("{a}");

    println!("Vector b:");
    println!("{b}");

    println!("We solve:");
    println!("A*x = b");

    // Solve A*x = b
    match a.lu().solve(&b) {
        Some(solution) => {
            let x = solution[0];
            let y = solution[1];

            println!();
            println!("--- Solution ---");
            println!("x = {x}");
            println!("y = {y}");

            // Verification
            let verification = a * solution;

            println!();
            println!("--- Verification ---");
            println!("A*x =");
            println!("{verification}");

            println!("b =");
            println!("{b}");
        }

        None => {
            println!();
            println!("The system does not have a unique solution.");
        }
    }
}
