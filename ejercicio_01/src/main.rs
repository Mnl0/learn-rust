use std::io;

fn main() {
    let mut temperatura = String::new();
    while temperatura.is_empty() {
        println!("Que deseas convertir");
        println!("Opcion 1 celsius a fahrenheit");
        println!("Opcion 2 fahrenheit a celsius");
    }
    io::stdin()
        .read_line(&mut temperatura)
        .expect("Failed to read line");
}

// 1. Convierte temperaturas entre Fahrenheit y Celsius.
// 2. Genera el enésimo número de Fibonacci.
// 3. Imprime la letra del villancico “Los Doce Días de Navidad”, aprovechando la repetición en la canción.
fn convercion_fahren_celsius() {
    let mut celsius = 0;
    let mut fahrenheit = 0;
}
