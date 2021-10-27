fn main() {
    value_char(5, 'm');
}

fn value_char(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}