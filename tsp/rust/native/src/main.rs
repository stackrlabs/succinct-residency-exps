use wasm::run_tsp;

fn main() {
    let graph = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];

    let result = run_tsp(graph);
    println!("{}", result);
}




