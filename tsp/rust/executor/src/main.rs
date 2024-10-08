use wasm::tsp;
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
