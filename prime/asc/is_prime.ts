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

// Returns 1 if n is prime, 0 otherwise
function is_prime(n: i64): i32 {
  if (n <= 1) {
    return 0;
  }
  if (n === 2) {
    return 0;
  }
  if (n % 2 === 0) {
    return 0;
  }

  for (let i = 3; i * i <= n; i += 2) {
    if (n % i === 0) {
      return 0;
    }
  }

  return 1;
}

export function zkmain(): void {
  let n = read_public_input();
  let claim = read_private_input();
  require(claim == is_prime(n));
}
