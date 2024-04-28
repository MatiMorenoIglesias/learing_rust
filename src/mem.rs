pub fn apply(){
    // Ownership tests
    //let str: String = String::from("PruebaStringOwnership.");
    //new_ownership_string(str);
    // println!("{}",str); FAIL!

    // let num: i32 = 1;
    // new_ownership_number(num);
    // println!("{}",num)

    // Ownership Return Test HEAP/STACK
    /* 
        let str: String = String::from("PruebaStringOwnership.");
        let returned_str = move_str_with_return(str);
        println!("{}",returned_str);
        // println!("{}",str); // FAIL!
        let num: i32 = 1;
        let returned_num = move_int_with_return(num);
        println!("num: {}.",num);
        println!("return_num: {}.",returned_num); 
    */

    // Ownership Reference Test
/*     
    let str: String = String::from("PruebaStringOwnership.");
    let len = reference_string(&str);
    println!("str len: {}.",len);
    println!("str: {}", str)
 */
    // Ownership with referenced var
    let mut str: String = String::from("PruebaStringOwnership.");
    let word: String = String::from(" words.");
    add_word_to_string(& mut str, word);
    let string_len = count_len_of_string(&str);
    println!("{}.", str);
    println!("{}.", string_len);
    /* 
    let string_var_1: String = String::from("string var n1");
    let string_var_2 = move_str_with_return(&string_var_1);
    println!("{}",string_var_1);
    let string = add_word_to_string(&mut string_var_1, "value");
    */
}
fn  count_len_of_string(value: &String) -> usize{
    return value.len();
}

fn new_ownership_number(value: i32){
    println!("{}", value);
}
fn new_ownership_string(value: String){
    println!("{}", value);
}

fn move_str_with_return(value: String) -> String{
    value
}

fn move_int_with_return(value: i32) -> i32 {
    value
}

fn reference_string(value: &String) -> usize{
    println!("{}", value);
    value.len()
}


fn add_word_to_string(str: &mut String, word: String) {
    str.push_str(&word);
} 


pub fn create_a_string() {
    let s: String = String::from("I'm inmutable string.");
    let mut s: String = String::from("I'm mutable string.");


    // Moving another data type using rust
    let var_1: i32 = 1;
    let var_2: i32 = var_1;

    // Replace string data using rust
    let str_var_1: String = String::from("I'm a string.");
    let str_var_2: String = str_var_1.clone() + " Cloned..";

    println!("{}", str_var_1);
    println!("{}", str_var_2);

}