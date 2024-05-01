/*===============ejemplo de mutabilidad===================*/
// fn main() {
//     let mut x = 5;
//     print!("The value of x is: {x} \n"); //\n salto de linea despues de imprimir
//     x = 6;
//     print!("The value of x is: {x} \n");
// }

/*==============ejemplo de constante==================== */
// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
// }

/*=============ejemplo de sombreado variable================== */
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         print!("The value of x in the inner scope is: {x} \n")
//     }
//     print!("The value of x is: {x} \n")
//sombreado permite cambiar tipo a diferencia de mut ya que creamos una nueva variable y utilizamos el nombre de la misma

// let spaces = "   "; //string
// let spaces = spaces.len(); //numerica

// let mut spaces = "   ";
// spaces = spaces.len(); //error del compilador al cambiar de tipo
// }

/*=======tio de datos =========== */
//tipo escalares
//enteros - numeros de punto flotante - booleanos - caracteres

fn main() {}
