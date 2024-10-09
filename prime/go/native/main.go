package main

import (
	"fmt"
	"os"
	"strconv"
)

func main() {
	// Parse command-line arguments
	if len(os.Args) != 2 {
		fmt.Println("Usage: go run main.go <number>")
		return
	}

	number, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	if isPrime(number) {
		fmt.Printf("Number %d is prime\n", number)
	} else {
		fmt.Printf("Number %d is not prime\n", number)
	}
}

func isPrime(n int) bool {
	if n <= 1 {
		return false
	}
	if n == 2 {
		return true
	}
	if n%2 == 0 {
		return false
	}
	for i := 3; i*i <= n; i += 2 {
		if n%i == 0 {
			return false
		}
	}
	return true
}
