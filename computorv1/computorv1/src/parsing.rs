use std::process;

use crate::polinomial::Polinomial;

pub fn parser (equation : &str)-> (Polinomial, Polinomial){
    if !equation.contains("="){
        println!("Equations should be separated by a \"=\"");
        process::exit(1);
    }
    let parts: Vec<&str> = equation.split("=").collect();
    if parts.len() < 1{
        println!("Equation Incomplete!");
        process::exit(1);
    }
    if parts.len() > 2{
        println!("Equation is not in valid form!");
        process::exit(1);
    }
    let lhs = poly_parser(parts[0].trim());
    let rhs = poly_parser(parts[1].trim());

    return (lhs, rhs);
}

fn preprocess_parser(textpoly: &str)-> String{
    let mut to_return = String::new();
    for i in textpoly.chars(){
        if i == '+' || i == '-' {
            to_return.push(' ');
            to_return.push(i);
            to_return.push(' ');
        }
        else {
            to_return.push(i);
        }
    }
    to_return
}

fn poly_parser(textpoly: &str)->Polinomial{
    let textpoly2 = preprocess_parser(textpoly);
    let monomials_text: Vec<&str> = textpoly2.split_whitespace().collect(); 
    let mut to_return = Polinomial::new( vec![0.0]);
    let mut cnt = 0;
    let mut mult = 1.0;
    for mon in monomials_text{
        if cnt % 2 == 0{
            let (coef, deg) = monomial_parser(mon.trim());
            to_return.set_coef(mult * coef, deg);
        }
        else {
            mult = match mon {
                "+" => 1.0,
                "-" => -1.0,
                _ => {
                    println!("Expected a sign: {}", mon);
                    std::process::exit(1);
                }
            }
        }
        cnt += 1;
    }
    // for mon in monomials_text{
    //     let (coef, deg) = monomial_parser(&mon);
    //     to_return.set_coef(coef,deg);
    // }
    return to_return;
}

fn monomial_parser(textmon: &str)-> (f64,usize){
    let coef;
    let text1: &str;
    if !textmon.contains("*") && !textmon.contains("^") && !textmon.contains("x"){
        return (textmon.parse().unwrap(),0);
    }
    if !textmon.contains("*"){
        coef = 1.0;
        text1 = textmon;
    }
     else{
        let split1: Vec<&str> = textmon.split("*").collect();
        if split1.len() != 2{
            println!("Wrongly formed term: {}", textmon);
            println!("Make sure to put spaces surrounding + and - operators");
            process::exit(1);
        }
        coef = split1[0].parse().unwrap();
        text1 = split1[1];
    }
    let deg;
    if  !text1.contains("^"){
        deg = 1;
    }
    else{
        let split2: Vec<&str> = text1.split("^").collect();
        if split2.len() != 2{
            println!("Wrongly formed monomial: {}", textmon);
            process::exit(1);
        }
        deg = split2[1].parse().unwrap();
    }
    return (coef,deg);
}