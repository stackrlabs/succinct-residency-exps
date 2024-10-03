fn main() {
    let graph = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    let n = 4;
    // bitmask for visited cities
    let visited = (1 << n) - 1;
    let r = visited + 1;
    let c = n;
    // Setup dp table
    let mut dp = vec![vec![-1; c]; r];

    let result = tsp(1, 0, &graph, &mut dp, n, visited);
    println!("{}", result);
}

// mask is the bitmask for visited cities
// pos is the current city
// graph is the adjacency matrix
// dp is the memoization table
// n is the number of cities
// visited is the bitmask for visited cities
fn tsp(mask: usize, pos: usize, graph: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, n: usize, visited: usize) -> i32 {
    if mask == visited {
        return graph[pos][0];
    }
    // If the result is already computed, return it
    if dp[mask][pos] != -1 {
        return dp[mask][pos];
    }

    let mut ans = i32::MAX;
    for city in 0..n {
        // If the city is not visited, compute the cost of visiting it
        if (mask & (1 << city)) == 0 {
            // Compute the cost of visiting the city and the cost of returning to the starting city
            let new = graph[pos][city] + tsp(mask | (1 << city), city, graph, dp, n, visited);
            ans = ans.min(new);
        }
    }

    // Store the result in the memoization table
    dp[mask][pos] = ans;
    dp[mask][pos]
}

