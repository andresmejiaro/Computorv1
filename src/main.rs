use std::env;
use std::process;

mod parsing;
mod polinomial;

fn main() {
    if env::args().count() != 2 {
        println!("Include your equation in a single argument");
        process::exit(1);
    }

    let uargs: Vec<String> = env::args().collect();

    let (lhs, rhs) = parsing::parser(&uargs[1]);
    let mut eq = lhs - rhs;
    println!("Reduced form: {} = 0", eq);
    eq.normalize();
    println!("Polinomial degree: {}", eq.degree());
    eq.solver_msg();
}
