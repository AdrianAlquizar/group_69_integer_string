fn main() {
    let number: i32 = 5;
    let string: String = "40".to_string();

    let number_string: String = number.to_string();
    let string_number: i32 = string.parse().unwrap();

    println!("Hello, world!");
}
