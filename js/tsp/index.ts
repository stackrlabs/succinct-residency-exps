function main(): void {
    const graph: number[][] = [
        [0, 10, 15, 20],
        [10, 0, 35, 25],
        [15, 35, 0, 30],
        [20, 25, 30, 0],
    ];
    const numCities: number = graph.length;
    // bitmask for visited cities
    const visited: number = (1 << numCities) - 1;
    const rows: number = visited + 1;
    // Setup dp table
    const dp: number[][] = Array(rows).fill(null).map(() => Array(numCities).fill(-1));

    const result: number = tsp(1, 0, graph, dp, numCities, visited);
    console.log(result);
}

// mask is the bitmask for visited cities
// currentCity is the current city
// graph is the distance matrix
// dp is the memoization table
// numCities is the number of cities
// visited is the bitmask for visited cities
function tsp(mask: number, currentCity: number, graph: number[][], dp: number[][], numCities: number, visited: number): number {
    if (mask === visited) {
        return graph[currentCity][0];
    }
    // If the result is already computed, return it
    if (dp[mask][currentCity] !== -1) {
        return dp[mask][currentCity];
    }

    let ans: number = Number.MAX_SAFE_INTEGER;
    for (let city = 0; city < numCities; city++) {
        // If the city is not visited, compute the cost of visiting it
        if ((mask & (1 << city)) === 0) {
            // Compute the cost of visiting the city and the cost of returning to the starting city
            const newCost: number = graph[currentCity][city] + tsp(mask | (1 << city), city, graph, dp, numCities, visited);
            ans = Math.min(ans, newCost);
        }
    }

    // Store the result in the memoization table
    dp[mask][currentCity] = ans;
    return dp[mask][currentCity];
}

main();
