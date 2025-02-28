pub fn extract_numbers(input: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<i32>() {
                numbers.push(num);
            }
            current_number.clear();
        }
    }

    // Check if there's a number left at the end of the string
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<i32>() {
            numbers.push(num);
        }
    }

    numbers
}