fn main() {
    println!("Hello, world!");
    bucle_loop();
    bucle_loop_dos();
    bucle_anidado();
    bucle_while();
    bucle_while_matriz();
    bucle_for();
    bucle_for_rev();
}

/*==========Ejemplo de bucles================ */
fn bucle_loop() {
    loop {
        println!("again!");
        break; //salir despues de una suposicion correcta
               //continue;
    }
}

fn bucle_loop_dos() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
/*=========ejemplo bucle anidado============ */
fn bucle_anidado() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
/*===========bucle while ================ */
fn bucle_while() {
    let mut number = 4;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
/*============recorriendo una matriz con while=============== */
fn bucle_while_matriz() {
    let matriz = [10, 20, 30, 40, 50, 60];
    let mut index = 0;

    while index < 6 {
        println!("The value is: {}", matriz[index]);

        index += 1;
    }
}
/*===============for==================== */
fn bucle_for() {
    let matriz = [10, 20, 30, 40, 50, 60];

    for element in matriz {
        println!("the value is: {element} in for");
    }
}
fn bucle_for_rev() {
    for number in (1..4).rev() {
        println!("{number} in for rev");
    }
    println!("LIFTOFF!!!");
}
