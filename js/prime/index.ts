function isPrime(num: number): boolean {
  if (num <= 1) {
    return false;
  }
  if (num === 2) {
    return true;
  }
  if (num % 2 === 0) {
    return false;
  }

  const upperLimit = Math.floor(Math.sqrt(num));
  for (let i = 3; i <= upperLimit; i += 2) {
    if (num % i === 0) return false;
  }

  return true;
}

const input = parseInt(prompt("Enter a number to check if it's prime:") || "");
if (isNaN(input)) {
  console.log("Invalid input. Please enter a valid number.");
} else {
  console.log(`${input} is ${isPrime(input) ? "prime" : "not prime"}.`);
}
