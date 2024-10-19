use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

fn main() {
    let number: i32 = 5;
    let string: String = "40".to_string();

    // Convert number (i32) to string
    let number_string: String = number.to_string();

    // Attempt to parse string (String) into a number (i32)
    let string_number: i32 = match string.parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to parse string to number: {}", e);
            return;
        }
    };

    // Print types and values
    println!("From number to string: {}", number_string);
    print_type_of(&number_string); // Print type of `number_string`

    println!("From string to number: {}", string_number);
    print_type_of(&string_number); // Print type of `string_number`
}

