/*========operaciones numericas==================== */
fn main() {
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let truncated = -5 / 3;

    let remainder = 43 & 5;

    print!("The values sum is: {sum} \n");
    print!("The values difference is: {difference} \n");
    print!("The values product is: {product} \n");
    print!("The values quotient is: {quotient} \n");
    print!("The values truncated is: {truncated} \n");
    print!("The values remainder is: {remainder} \n");

    let valor_op_bolean = op_boleanos();
    print!("El valor que retorna mi funcion es: {} \n", valor_op_bolean);

    /*extrayendo los valores de una funcion */
    let (valor_t, valor_f) = op_boleanos_tuplas();
    print!("El valor de f es: {} \n", valor_f);
    print!("El valor de t es: {} \n", valor_t);

    let (valor_c, valor_z, valor_gato) = tipo_char();
    print!("El valor de c es : {valor_c}, El valor de z es : {valor_z}, El valor de gato es : {valor_gato} \n");

    let valor_tupla = tipo_tupla();
    print!("El valor de la tupla es : {:?} \n", valor_tupla);

    /*coincidencia de patrones para destructurar tupla */
    let (x, y, z) = tipo_tupla();
    print!("El valor x es: {x}, el valor de y es: {y}, el valor de z es: {z} \n");
    /*tambien podemos acceder a un valor usando un punto */
    let x: (i32, f64, u8) = (1000, 7.5, 12);
    let primer_elemento = x.0;
    println!(
        "El valor del primer elemento de la tupla es {}",
        primer_elemento
    );

    let valor_matriz = tipo_matriz();
    println!("Matriz devuelta: {:?}", valor_matriz);
    let valor_matriz_dos = tipo_matriz_dos();
    println!("Matriz devuelta dos: {:?}", valor_matriz_dos);
    let valor_matriz_string = tipo_matriz_string();
    println!("Matriz devuelta string: {:?}", valor_matriz_string);
}

/*========operador boleano ========== */
fn op_boleanos() -> bool {
    let t = true;
    t
}

/*==========devolver mas de un valor en una funcion con tuplas==================== */
fn op_boleanos_tuplas() -> (bool, bool) {
    let t = true;
    let f: bool = false;
    (t, f)
}

/*=========tipo de dato char==========================*/
fn tipo_char() -> (char, char, char) {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    (c, z, heart_eyed_cat)
}

/*==========tipo compuesto tupla====================*/
fn tipo_tupla() -> (i32, f64, u8) {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    tup
}
/*==========tipo dato matriz======================== */
fn tipo_matriz() -> [i32; 5] {
    let a = [1, 2, 3, 4, 5];
    a
}

fn tipo_matriz_dos() -> [[i32; 3]; 2] {
    let matriz: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]]; //2 matrices de 3 elementos
    matriz
}

fn tipo_matriz_string() -> [&'static str; 12] {
    //se debe declarar que es estatica durante toda la ejecucion no cambia de preferencia Vec<String>
    let matriz: [&'static str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    matriz
}

//NOTAS
/*Se recomienda utilizar snake_case para variables, funciones e identificadores */
/*Se recomienda utilizar matrices cuando se sabe el largo de los elementos en caso contrario utilizar un vector */
