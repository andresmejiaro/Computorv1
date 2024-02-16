use std::process;

use crate::polinomial::Polinomial;

struct ParserState{
    coef: f64,
    coef_sign: f64,
    caret_appeared: bool,
    x_apeared: bool,
    power: usize,
    mult_apeared: bool,
    coef_parsed: bool,
    power_parsed: bool,
}


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
    to_return.push_str("| ");
    for i in textpoly.chars(){
        if i == '+' || i == '-' {
            to_return.push(' ');
            to_return.push('|');
            to_return.push(' ');
            to_return.push(i);
            to_return.push(' ');
        }
        else if i == '*' || i == '^' || i == 'X' {
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
    //println!("{textpoly2}");
    let monomials_text: Vec<&str> = textpoly2.split_whitespace().collect(); 
    let mut to_return = Polinomial::new( vec![0.0]);
    let mut parser_state = ParserState {coef: 0.0, coef_sign: 1.0,
        caret_appeared: false,x_apeared: false,  power: 0, mult_apeared: false, 
        coef_parsed: true, power_parsed:false};
    for mon in monomials_text{
        match  mon {
            "|" => {
                if !parser_state.coef_parsed {
                    println!("There seems to be repeated signs or other unexpected behavior");
                    parse_error(textpoly, "")
                }

                update_polynomial(&parser_state, &mut to_return);
                initialize_parser_state(&mut parser_state);
            }
            "+" => {
                if parser_state.caret_appeared || parser_state.mult_apeared ||
                parser_state.x_apeared || parser_state.coef_parsed {
                    parse_error(textpoly, mon);
                }
            }
            "-" => {
                if parser_state.caret_appeared || parser_state.mult_apeared ||
                parser_state.x_apeared || parser_state.coef_parsed{
                    parse_error(textpoly, mon);
                }
                parser_state.coef_sign *= -1.0;
            }
            "X" => {
                if parser_state.caret_appeared || parser_state.x_apeared
                {
                    parse_error(textpoly, mon);
                }
                if !parser_state.coef_parsed{
                    parser_state.coef = 1.0;
                    parser_state.coef_parsed = true;
                }
                parser_state.power = 1;
                parser_state.x_apeared = true;
            }
            "^" =>{
                if !parser_state.x_apeared || parser_state.caret_appeared
                {
                    parse_error(textpoly, mon);
                }
                parser_state.caret_appeared = true;
                
            }
            "*" =>{ 
                if !parser_state.coef_parsed || parser_state.mult_apeared{
                    parse_error(textpoly, mon);
                }
                parser_state.mult_apeared = true;
            }
            mon => {
                if mon.chars().all(|c| c.is_digit(10) || c == '.'){
                    if !parser_state.caret_appeared && !parser_state.coef_parsed{
                        parser_state.coef = match mon.parse::<f64>(){
                            Ok (num) => num,
                            Err(_) => {
                                println!("Failing to parse {mon} as a coefficient");
                                parse_error(textpoly, mon);
                                1.0
                            }
                        };
                        parser_state.coef_parsed = true;
                    } else if !parser_state.caret_appeared && parser_state.coef_parsed{
                        parse_error(textpoly, mon);
                    } else if !parser_state.power_parsed {
                        parser_state.power = match mon.parse::<usize>() {
                            Ok (num) => num,
                            Err(_) => {
                                println!("Failing to parse {mon} as an exponent");
                                parse_error(textpoly, mon);
                                1
                            }
                        };
                        parser_state.power_parsed = true;
                    } else {
                        parse_error(textpoly, mon);
                    }
                } else {
                    parse_error(textpoly,mon);
                }
            }
        }
    }

    update_polynomial(&parser_state, &mut to_return);
    return to_return;
}

fn parse_error(textpoly: &str, mon: &str){
    println!("Parsing error in {textpoly} near {mon}");
    std::process::exit(1);
}


fn initialize_parser_state(parser_state: &mut ParserState){
    parser_state.coef = 0.0;
    parser_state.coef_sign = 1.0;
    parser_state.caret_appeared = false;
    parser_state.x_apeared = false;
    parser_state.power = 0;
    parser_state.mult_apeared = false;
    parser_state.coef_parsed =false;
    parser_state.power_parsed = false;

}

fn update_polynomial(parser_state: &ParserState, polinomial: &mut Polinomial){
    polinomial.update_coef(parser_state.coef*parser_state.coef_sign, parser_state.power);
    
}
