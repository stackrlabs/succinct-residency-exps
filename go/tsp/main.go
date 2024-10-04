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
	numCities := len(graph)
	// bitmask for visited cities
	visited := (1 << numCities) - 1
	rows := visited + 1
	// Setup dp table
	dp := make([][]int, rows)
	for i := range dp {
		dp[i] = make([]int, numCities)
		for j := range dp[i] {
			dp[i][j] = -1
		}
	}

	result := tsp(1, 0, graph, dp, numCities, visited)
	fmt.Println(result)
}

// mask is the bitmask for visited cities
// currentCity is the current city
// graph is the distance matrix
// dp is the memoization table
// n is the number of cities
// visited is the bitmask for end state when all cities are visited
func tsp(mask, currentCity int, graph [][]int, dp [][]int, n, visited int) int {
	if mask == visited {
		return graph[currentCity][0]
	}
	// If the result is already computed, return it
	if dp[mask][currentCity] != -1 {
		return dp[mask][currentCity]
	}

	minCost := math.MaxInt32
	for city := 0; city < n; city++ {
		// If the city is not visited, compute the cost of visiting it
		if (mask & (1 << city)) == 0 {
			// Compute the cost of visiting the city and the cost of returning to the starting city
			visitCost := graph[currentCity][city] + tsp(mask|(1<<city), city, graph, dp, n, visited)
			minCost = min(minCost, visitCost)
		}
	}

	// Store the result in the memoization table
	dp[mask][currentCity] = minCost
	return dp[mask][currentCity]
}
