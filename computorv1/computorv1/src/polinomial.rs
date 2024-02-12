use std::str::FromStr;

#[derive(PartialEq, Clone)]
pub struct Polinomial
{
    coefficients: Vec<f64>,
}

impl Polinomial
{
    pub fn new(coefficients: Vec<f64>) -> Self {
        let mut poly = Polinomial {coefficients};
        poly.simplify_grade();
        return poly;
    }

    pub fn set_coef(&mut self, coef: f64, grade: usize ) {
        if grade + 1 > self.coefficients.len(){
            self.coefficients.resize(grade + 1, 0.0);
        }
        self.coefficients[grade] = coef;
    }

    pub fn degree(&self) -> usize{
        return self.coefficients.len() - 1;
    }

    pub fn simplify_grade(&mut self){
        if self.coefficients.ends_with(&[0.0]) && 
                self.coefficients.len() > 1{
            self.coefficients.pop();
            self.simplify_grade();
        }
    }

    fn solve_1(&self) -> f64{
        return -self.coefficients[0]/ self.coefficients[1];
    }

    fn solver_msg2(&self){
        if self.degree() != 2{
            println!("Something was very wrong. Solver 2 with nonconforming");
            std::process::exit(1);
        }
        let discr = self.coefficients[1] * self.coefficients[1] - 
            4.0 * self.coefficients[0] *self.coefficients[2];
        if discr < 0.0{
            println!("Discriminant is negative, the solutions are complex:");          
            let sol1 = Polinomial::new(vec![discr,0.0, 1.0]);
            let impart = sol1.newton(-discr);
            println!("{:.3} + {:.3}i",-0.5* self.coefficients[1],impart);
            println!("{:-.3} - {:.3}i",-0.5* self.coefficients[1],impart);
        }  else if discr == 0.0 {
            println!("Discriminant is zero, the solution is:");          
            println!("{}:", -0.5* self.coefficients[1]);           
        } else {
            println!("Discriminant is strictly positive, the two solutions are:");
            println!("{:.3}", self.newton(-0.5 * self.coefficients[1]+discr));
            println!("{:.3}", self.newton(-0.5 * self.coefficients[1]-discr));
        }
    }

    fn high_level_solver(&self){
        println!("The polinomial degree is strictly greater than 2. I can't solve.");
        println!("... I mean we can try ...");
        println!("Give me a number to start searching, all kind of things can go wrong in");
        println!("a search, we may not get to a solution, we may get NaN, we may loop");
        loop{
            println!("Give me a number to start the search leave empty to exit");
            let mut input_string = String::new();
            match io::stdin().read_line(&mut input_string){
                Ok(_) => {
                    
                },
                Err(_e) => {
                    println!("Error reading line.");
                    continue;
                }
            }
            let trimmed = input_string.trim();
            if trimmed.is_empty(){
                println!("Bye!");
                break;
            }
            let start_point = match f64::from_str(trimmed){
                Ok(num) => num,
                Err(_) =>{
                    println!("Invalid input. Please enter a valid number");
                    continue;
                }
            };
            println!("Solving equation {} = 0", self);
            println!("Starting with newton iterations at {:.3}", start_point);
            let solved = self.newton(start_point);
            println!("I get {:.3}", solved);
            println!("the polynomial evaluated at that point is: {:.3}", self.evaluate(solved));
            println!("IF the value above is (very close to) zero we found a solution!");

        }
    }

    pub fn solver_msg(&self){
        match self.degree() {
            0 => {
                if self.coefficients[0] != 0.0 {
                    println!(
                      "This equation is inconsistant. No values satisfy it");
                }
                else {
                    println!(
                        "This equation is a tautology. All values satisfy it");
                }
            }
            1 => {
                println!("The solution is:");
                println!("{}", self.solve_1());   
            }
            2 => {self.solver_msg2()}
            _ => {self.high_level_solver()}
            
        }
    }

    pub fn derivative(&self)-> Polinomial{
        let mut to_return = Polinomial::new( vec![0.0]);
        for (index,coef) in   self.coefficients.iter().enumerate(){
            if index > 0 {
                to_return.set_coef(index as f64 * coef, index - 1);
            }
        }
        return to_return;
    }

    pub fn evaluate(&self, value: f64)->f64{
        let mut acum = 0.0;
        for (grade, coef) in self.coefficients.iter().enumerate() {
            let mut pow = 1.0;
            for _i in 0..grade {
                pow *= value;
            }
            acum += pow*coef;
        }
        return  acum;
    }

    pub fn newton(&self, start_point: f64)-> f64{
        let mut current = start_point;
        let deriv = self.derivative();
        for it in 1..1000 {
            let ev = self.evaluate(current);
            if ev*ev < 1.0e-10{
               break
            }
            current = current - self.evaluate(current)/deriv.evaluate(current);
            if it == 1000 {
                println!("Diverges!");
            }  
        }
        return current;
    }

    pub fn normalize(&mut self){
        self.simplify_grade();
        if self.coefficients.len() < 1{
            return;
        }
        if self.coefficients.len() == 1 && self.coefficients[0] == 0.0{
            return;
        }
        let val = 1.0/self.coefficients.last().unwrap();
        for it in self.coefficients.iter_mut(){
            *it *= val;
        }
    }

}


use std::io;
use std::ops::Add;

impl Add for Polinomial
{
    type Output = Polinomial;
    fn add(self, other: Self) -> Polinomial {
        let mut acoef = self.coefficients;
        let mut bcoef: Vec<f64> = other.coefficients;
        if acoef.len() < bcoef.len(){
            acoef.resize(bcoef.len(), 0.0);
        } else {
            bcoef.resize(acoef.len(), 0.0);
        }
        let acoef = acoef.iter().zip(bcoef.iter()).
                                map(|(&x, &y)| x + y).collect();

        return Self::new(acoef);
    }
}

use std::ops::Sub;

impl Sub for Polinomial
{
    type Output = Polinomial;
    fn sub(self, other: Self) -> Polinomial {
        let mut acoef = self.coefficients;
        let mut bcoef: Vec<f64> = other.coefficients;
        if acoef.len() < bcoef.len(){
            acoef.resize(bcoef.len(), 0.0);
        } else {
            bcoef.resize(acoef.len(), 0.0);
        }
        let acoef = acoef.iter().zip(bcoef.iter()).
                                map(|(&x, &y)| x - y).collect();

        return Polinomial::new(acoef);
    }
}

use std::fmt;

impl fmt::Display for Polinomial
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        let len = self.coefficients.len();
        if len == 1 && self.coefficients[0] == 0.0{
            write!(f,"0")
        } else {
            for (rev_index, coef) in 
                    self.coefficients.iter().rev().enumerate(){
                let index = len - 1 - rev_index;
                if *coef == 0.0 {
                    continue;
                }
                let mut adj = 1.0;
                if !result.is_empty(){
                    if *coef < 0.0{
                        result.push_str(" - ");
                        adj = -1.0;
                    } else {
                    result.push_str(" + ")
                    }
                }
                if index == 0{
                    result.push_str(&format!("{}", adj*coef));
                } else if adj*coef != 1.0 {
                result.push_str(&format!("{}*x^{}", adj*coef, index));  
                } else {
                    result.push_str(&format!("x^{}", index));  

                }

            }
            write!(f,"{}", result)
        }
    }
}

 