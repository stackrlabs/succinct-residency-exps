use bytemuck::cast_slice;

#[no_mangle]
extern "C" fn binary_search(data_ptr: *const i32, count: i32, target: i32) -> i32 {
    let list = read_list(data_ptr, count);
    let mut left = 0;
    let mut right = list.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if list[mid] == target {
            return 1;
        }
        if list[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}

// Reads list from linear memory
fn read_list(data_ptr: *const i32, count: i32) -> Vec<i32> {
    use core::slice;
    let ptr = data_ptr as *const u8;
    let data: Vec<u8> = unsafe { slice::from_raw_parts(ptr, (count*4) as usize).to_vec() };
    bytemuck::cast_slice(&data).to_vec()
}
