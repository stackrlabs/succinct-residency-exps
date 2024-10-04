#[no_mangle]
pub fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }

    if number == 2 {
        return true;
    }

    // Filter out even numbers
    if number % 2 == 0 {
        return false;
    }
    // Only need to check up to the square root of the number
    let limit = (number as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        if number % i == 0 {
            return false;
        }   
    }
    true
}
