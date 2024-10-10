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
    // let limit = (number as f64).sqrt() as i32;
    // sqrt on f64 throws 'not yet implemented', so we can break the loop when i * i > number
    for i in (3..=number).step_by(2) {
        if number % i == 0 {
            return false;
        }
        if i * i > number {
            break;
        }
    }
    true
}
