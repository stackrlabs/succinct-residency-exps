package main

import (
	"fmt"
	"os"
	"strconv"
)

func main() {
	// Parse command-line arguments
	if len(os.Args) != 2 {
		fmt.Println("Usage: go run main.go <prime-number>")
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

func isPrime(number int) bool {
	if number <= 1 {
		return false
	}
	if number == 2 {
		return true
	}
	if number%2 == 0 {
		return false
	}
	for i := 3; i*i <= number; i++ {
		if number%i == 0 {
			return false
		}
	}
	return true
}
