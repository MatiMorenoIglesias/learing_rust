//use std::result;
mod guess;

const SOY_UNA_CONSTANTE: i8 = 1;
fn main() {
    //inmutabilidad();
    //shadowing();
    //println!("El resultado es: {}",calcular_factorial_fp(3));
    //println!("El resultado es: {}",calcular_factorial_normal(3));
    guess::guess_name()
}

fn calcular_factorial_fp(number: u128) -> u128 {
    if number == 0 || number == 1 { 1 } 
    else {
        return calcular_factorial_fp(number - 1) * number;
    }
}

fn calcular_factorial_normal(number: u128) -> u128 {
    if number == 0 || number == 1 { 1 }
    else {
        let mut rs: u128 = number;
        for i in (1..number).rev(){
            rs = rs * i;
        }
        return rs
    }
}

fn inmutabilidad(){
    let _x: i32 = 1;
    println!("{}",_x)
}

fn shadowing(){
    let x: i8 = 1;
    let x: i8 = x * 10;
    let x: i8 = x * 2;
    println!("{}", x)
}