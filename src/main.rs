//use std::result;
mod guess;
mod mem;
mod structs;
mod errors;

const SOY_UNA_CONSTANTE: i8 = 1;

fn show_name(name: &String) {
    println!("{}.",name);
}

fn main() {
    /* let name: String = String::from("Matias");
    let customer = structs::Customer::new(name);
    show_name(customer.get_name()); */


    //guess::guess_name();
    //errors::bad_index();
    errors::open_unexisting_file();
    

    //println!("My name is {} and my state is {}.", customer.name, customer.is_active());
    //inmutabilidad();
    //shadowing();
    //println!("El resultado es: {}",calcular_factorial_fp(3));
    //println!("El resultado es: {}",calcular_factorial_normal(3));
    //guess::guess_name();
    //mem::create_a_string();
    //mem::apply();


    // Option and Some
    let fav_num: Option<i8> = Some(13);
    match fav_num {
        Some(val) => {
            println!("fav number: {}.", val);
        },
        _ => {
            println!("No hay ningun valor.");
        }
    }
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