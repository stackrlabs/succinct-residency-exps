package main

import (
	"fmt"
	"math"
)

func main() {
	graph := [][]int{
		{0, 10, 15, 20},
		{10, 0, 35, 25},
		{15, 35, 0, 30},
		{20, 25, 30, 0},
	}
	n := 4
	// bitmask for visited cities
	visited := (1 << n) - 1
	r, c := visited+1, n
	// Setup dp table
	dp := make([][]int, r)
	for i := range dp {
		dp[i] = make([]int, c)
		for j := range dp[i] {
			dp[i][j] = -1
		}
	}

	result := tsp(1, 0, graph, dp, n, visited)
	fmt.Println(result)
}

// mask is the bitmask for visited cities
// pos is the current city
// graph is the adjacency matrix
// dp is the memoization table
// n is the number of cities
// visited is the bitmask for visited cities
func tsp(mask, pos int, graph [][]int, dp [][]int, n, visited int) int {
	if mask == visited {
		return graph[pos][0]
	}
	// If the result is already computed, return it
	if dp[mask][pos] != -1 {
		return dp[mask][pos]
	}

	ans := math.MaxInt32
	for city := 0; city < n; city++ {
		// If the city is not visited, compute the cost of visiting it
		if (mask & (1 << city)) == 0 {
			// Compute the cost of visiting the city and the cost of returning to the starting city
			new := graph[pos][city] + tsp(mask|(1<<city), city, graph, dp, n, visited)
			ans = min(ans, new)
		}
	}

	// Store the result in the memoization table
	dp[mask][pos] = ans
	return dp[mask][pos]
}
