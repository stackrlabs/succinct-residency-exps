@external("env", "wasm_input")
declare function wasm_input(x: i32): i64

@external("env", "require")
declare function require(x: i32): void

export function read_public_input(): i64 {
  return wasm_input(1);
}

export function read_private_input(): i64 {
  return wasm_input(0);
}

export function abort(message: string | null, fileName: string | null, lineNumber: u32, columnNumber: u32): void {
  let a = 0;
  a++;
}

function binary_search(list: i64[], number: i64): i32 {
  let low = 0;
  let high = list.length - 1;
  while (low <= high) {
    let mid = Math.floor(low + (high - low) / 2) as i32;
    if (list[mid] === number) {
      return 1;
    }
    if (list[mid] < number) {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return 0;
}

export function zkmain(): void {
  let number = read_public_input();
  // read from inputs like this 1:i64,0:i64 one by one
  let list: i64[] = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
  require(binary_search(list, number));
}
