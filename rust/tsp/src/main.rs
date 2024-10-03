fn main() {
    let graph = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    let num_of_cities = graph.len();
    // bitmask for visited cities
    let visited = (1 << num_of_cities) - 1;
    let rows: usize = visited + 1;
    // Setup dp table
    let mut dp = vec![vec![-1; num_of_cities]; rows];

    let result = tsp(1, 0, &graph, &mut dp, num_of_cities, visited);
    println!("{}", result);
    println!("{:?}", dp);
}

// mask is the bitmask for visited cities
// pos is the current city
// graph is the distance matrix
// dp is the memoization table
// n is the number of cities
// visited is the bitmask for visited cities
fn tsp(mask: usize, current_city: usize, graph: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, n: usize, visited: usize) -> i32 {
    if mask == visited {
        return graph[current_city][0];
    }
    // If the result is already computed, return it
    if dp[mask][current_city] != -1 {
        return dp[mask][current_city];
    }

    let mut min_cost = i32::MAX;
    for city in 0..n {
        // If the city is not visited, compute the cost of visiting it
        if (mask & (1 << city)) == 0 {
            // Compute the cost of visiting the city and the cost of returning to the starting city
            // Note: this always leaves the even rows unchanged in the `dp` table, as we have last bit set. This can be optimized but left for simplicity
            let visit_cost = graph[current_city][city] + tsp(mask | (1 << city), city, graph, dp, n, visited);
            min_cost = min_cost.min(visit_cost);
        }
    }

    // Store the result in the memoization table
    dp[mask][current_city] = min_cost;
    dp[mask][current_city]
}

