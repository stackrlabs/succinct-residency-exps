package main

import (
	"fmt"
	"math/rand"
	"os"
	"sort"
	"strconv"
)

func main() {
	// Parse command-line arguments
	if len(os.Args) != 3 {
		fmt.Println("Usage: go run main.go <number> <length>")
		return
	}

	number, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	length, err := strconv.Atoi(os.Args[2])
	if err != nil {
		fmt.Println("Invalid length:", os.Args[2])
		return
	}

	// Create a random list of integers
	list := make([]int, length)
	for i := 0; i < length; i++ {
		list[i] = rand.Intn(length) // Generate random numbers based on list length
	}

	// Sort the list (required for binary search)
	sort.Ints(list)

	fmt.Println("Generated list:", list)

	// Perform binary search
	index := binarySearch(list, number)

	if index != -1 {
		fmt.Printf("Number %d found at index %d\n", number, index)
	} else {
		fmt.Printf("Number %d not found in the list\n", number)
	}
}

func binarySearch(list []int, target int) int {
	left := 0
	right := len(list) - 1

	for left <= right {
		mid := (left + right) / 2
		if list[mid] == target {
			return mid
		}
		if list[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return -1
}
