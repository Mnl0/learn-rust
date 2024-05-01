fn main() {
    // println!("Hello, world!");

    // another_functions(5);

    print_labeled_measurement(5, 'h');

    another_functions(32);
}

fn another_functions(x: i32) {
    println!("The value of is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
