package main

import (
	"crypto/sha256"
	"fmt"
	"os"
	"strconv"
)

func main() {
	// Parse command-line arguments
	if len(os.Args) != 2 {
		fmt.Println("Usage: go run main.go <num-leaves>")
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

func merkelize(leaves []string) string {
	// Check if the number of leaves is a power of 2
	if len(leaves) <= 0 || (len(leaves)&(len(leaves)-1)) != 0 {
		fmt.Println("Error: Number of leaves must be a positive power of 2")
		return ""
	}

	// Build the Merkle tree
	return buildMerkleTree(leaves)
}

func buildMerkleTree(nodes [][]byte) string {
	if len(nodes) == 1 {
		return nodes[0]
	}

	var newLevel []string
	for i := 0; i < len(nodes); i += 2 {
		left := nodes[i]
		right := nodes[i+1]
		newNode := hashPair(left, right)
		newLevel = append(newLevel, newNode)
	}

	return buildMerkleTree(newLevel)
}

func hashPair(left, right string) string {
	combined := left + right
	hash := sha256.Sum256([]byte(combined))
	return fmt.Sprintf("%x", hash)
}
