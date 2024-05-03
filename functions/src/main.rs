fn main() {
    // println!("Hello, world!");

    // another_functions(5);

    print_labeled_measurement(5, 'h');

    another_functions(32);

    let duplicar_valor = ejecutar_fn(duplicar, 5);
    println!("El valor duplicado es: {}", duplicar_valor);

    let tripicar_valor = ejecutar_fn(triplicar, 10);
    println!("El valor duplicado es: {}", tripicar_valor);

    ejemplo_declaracion_expresion(3);
    //let decl = 6; //declaracion
}

fn another_functions(x: i32) {
    println!("The value of is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
/*=============pasar funcion como argumento==============*/
fn ejecutar_fn(funcion: fn(i32) -> i32, valor: i32) -> i32 {
    funcion(valor)
}

//declaracion de tipo de retorno
fn duplicar(numero: i32) -> i32 {
    numero * 2
}

fn triplicar(numero: i32) -> i32 {
    numero * 3
}
/// comentario de documentacion sirve para documentar la funcion admite Markdown
///
/// #Ejemplo
/// ```
/// let arg = 5;
/// let valueFuncion = ejemplo_declaracion_expresion(arg);
/// ```
///
fn ejemplo_declaracion_expresion(numero: i32) {
    let y = {
        let x = numero;
        x + 1 //expresion
    };
    println!("The value of is: {y}");
}
//NOTAS:
//Se de indicar que tipo de argumento recibira la funcion eje i32, char, str
//Las declaraciones son instrucciones que realizan alguna acción y no devuelven un valor.
//Las expresiones se evalúan como un valor resultante.
//las expresiones no deben llevar ';' ya que el compilador la interpreta como declaracion
