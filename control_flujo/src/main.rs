fn main() {
    if_expresion(1);
    if_let_declaracion();
}
/*===========ejemplo ifexpresion================ */
fn if_expresion(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
/*============usar if en un let declaracion================ */
fn if_let_declaracion() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
