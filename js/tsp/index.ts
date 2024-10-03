function main(): void {
    const graph: number[][] = [
        [0, 10, 15, 20],
        [10, 0, 35, 25],
        [15, 35, 0, 30],
        [20, 25, 30, 0],
    ];
    const n: number = 4;
    // bitmask for visited cities
    const visited: number = (1 << n) - 1;
    const r: number = visited + 1;
    const c: number = n;
    // Setup dp table
    const dp: number[][] = Array(r).fill(null).map(() => Array(c).fill(-1));

    const result: number = tsp(1, 0, graph, dp, n, visited);
    console.log(result);
}

// mask is the bitmask for visited cities
// pos is the current city
// graph is the adjacency matrix
// dp is the memoization table
// n is the number of cities
// visited is the bitmask for visited cities
function tsp(mask: number, pos: number, graph: number[][], dp: number[][], n: number, visited: number): number {
    if (mask === visited) {
        return graph[pos][0];
    }
    // If the result is already computed, return it
    if (dp[mask][pos] !== -1) {
        return dp[mask][pos];
    }

    let ans: number = Number.MAX_SAFE_INTEGER;
    for (let city = 0; city < n; city++) {
        // If the city is not visited, compute the cost of visiting it
        if ((mask & (1 << city)) === 0) {
            // Compute the cost of visiting the city and the cost of returning to the starting city
            const newCost: number = graph[pos][city] + tsp(mask | (1 << city), city, graph, dp, n, visited);
            ans = Math.min(ans, newCost);
        }
    }

    // Store the result in the memoization table
    dp[mask][pos] = ans;
    return dp[mask][pos];
}

main();
