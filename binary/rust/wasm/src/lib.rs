#[no_mangle]
pub fn binary_search(list: Vec<i32>, target: i32) -> bool {
    let mut left = 0;
    let mut right = list.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if list[mid] == target {
            return true;
        }
        if list[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}
