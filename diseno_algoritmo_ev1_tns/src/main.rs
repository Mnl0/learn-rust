use std::io;

fn main() {
    ejercicio_uno();
}

fn ejercicio_uno() {
    // Desarrolle un algoritmo que permita a un usuario ingresar por separado su nombre, su
    // apellido paterno y su apellido materno, su año de nacimiento (Ej. 1990), y su sueldo Bruto.
    // Posteriormente mostrar por pantalla los datos, a manera del siguiente ejemplo calculando
    // su sueldo Líquido.
    // El sueldo líquido se calcula en base al Sueldo Bruto menos el descuento del 20% más el
    // 15% de Bonos. Calcule la edad solo en base al año actual.
    println!("Ingresar Nombre");
    let mut nombre: String = String::new();
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al ingresar el nombre");

    println!("Ingresar Apellido paterno");
    let mut apellido_paterno: String = String::new();
    io::stdin()
        .read_line(&mut apellido_paterno)
        .expect("Error al ingresar el Apellido Paterno");

    println!("Ingresar Apellido materno");
    let mut apellido_materno: String = String::new();
    io::stdin()
        .read_line(&mut apellido_materno)
        .expect("Error al ingresar el Apellido matero");

    println!("Ingresar Fecha de nacimiento");
    let mut fecha_nacimiento: String = String::new();
    io::stdin()
        .read_line(&mut fecha_nacimiento)
        .expect("Error al ingredar fecha de nacimiento");

    println!("Ingresar sueldo bruto");
    let mut sueldo_bruto: String = String::new();
    io::stdin()
        .read_line(&mut sueldo_bruto)
        .expect("Error al ingresar el sueldo");

    let fecha_nacimiento: i32 = match fecha_nacimiento.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    let sueldo_bruto: f32 = match sueldo_bruto.trim().parse() {
        Ok(num) => num,
        Err(_) => -1.0,
    };
    let descuento = sueldo_bruto * 0.20;
    let bono = sueldo_bruto * 0.15;
    let sueldo_liquido = sueldo_bruto + bono - descuento;
    let edad = 2024 - fecha_nacimiento;

    println!("Nombre : {nombre} {apellido_paterno} {apellido_materno}");
    println!("Edad: {edad}");
    println!("Sueldo Bruto: ${sueldo_bruto}");
    println!("Descuento 20%: ${descuento}");
    println!("Bono 15%: %{bono}");
    println!("Sueldo Liquido: ${sueldo_liquido}");
}

fn ejercicio_dos() {
    // Una Circo desea implementar un sistema para cobrar entradas. El cliente debe indicar
    // cuantas entradas desea e indicar el tipo de entrada que comprará
    // Las tarifas son las siguientes:
    // Tipo de Persona Valor Edad
    // Niños $2.500 De 0 a 12 años
    // Adultos $4.000 De 13 a 59 años
    // Adulto Mayor $2.000 De 60 en adelante
    // Dada la solicitud, desarrolle el código que permita determinar el total a pagar de acuerdo
    // con el tipo solicitada y la cantidad de entradas.
    // Se debe mostrar la siguiente información de salida:
    // Niños: 2
    // Adultos: 1
    // Adulto Mayor 1
    // TOTAL A PAGAR: $11000

    print!("Bienvenido al Ciro du solei");
    print!("Ingrese el numero de entradas a comprar");
}

fn ejercicio_tres() {
    // Película HORARIO
    // 1. La quebrazón de platos de goma 16:00
    // 2. La fuga del camión sin ruedas 18:00
    // 3. La esquina de la mesa redonda 22:00
    // El menú debe permitir a un usuario seleccionar una opción. En caso de que la opción no sea válida,
    // debe mostrar el mensaje de error y volver a preguntar.
    // Todas las entradas valen $3500. Finalmente muestre un mensaje con el resumen de la compra como
    // el ejemplo siguiente:
    // La quebrazón de platos de goma 16:00 Hrs.
    // Cantidad de entradas 2
    // Total a pagar $7000
}
