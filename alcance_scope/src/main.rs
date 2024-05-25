fn main() {
    //s no es valido aqui, aun no esta declarado
    let _s = "Hello"; //s es valido desde aqui hasta el final del bloque
    from_function();
    from_function_mut();
    string_clone();
    copiar_pila();
    valores_y_retornos();
    devolver_tupla();
} //se ejecuta funcion drop

fn from_function() {
    let _s = String::from("hello");
    //::permite asigar un espacio de nombre a la funcion
}

fn from_function_mut() {
    let mut s = String::from("hello");
    s.push_str(",world!"); //agrega texto a la cadena
    println!("{}", s) //imprime "hello, world!"
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    //ambas variables apuntan a la misma memoria
    println!("{}, world!", s1); //error, s1 ya no es valido
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn copiar_pila() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn valores_y_retornos() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}", s1);
    //println!("s1 = {}", s2); error
    println!("s3 = {}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn devolver_tupla() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
