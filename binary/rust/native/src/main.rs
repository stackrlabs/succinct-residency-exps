use wasm::binary_search;
use clap::Parser;
use rand;
use binary::binary_search;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    number: i32,
    #[arg(short, long)]
    length: i32,
}

fn main() {
    // let args = Args::parse();
    let length = 300000;
    let mut list = generate_random_list(length, length);
    list.sort();
    println!("List: {:?}", list);
    use std::fs::File;
    use std::io::Write;
    use serde_json::json;

    // Choose a random search value from the list
    let search_value = list[rand::random::<usize>() % list.len()];

    // Create a JSON object
    let json_data = json!({
        "list": list,
        "value": search_value
    });

    // Write JSON to file
    let mut file = File::create("/Users/prudhvirampey/Documents/stackrlabs/succinct-residency-exps/inputs/binary.json").expect("Failed to create file");
    file.write_all(json_data.to_string().as_bytes()).expect("Failed to write to file");

    println!("JSON data saved to inputs/binary.json");
    println!("Search value: {}", search_value);
    // let found = binary_search(list, arg÷s.number);
    // println!("Element found?: {}", found);
}

fn generate_random_list(length: i32, range: i32) -> Vec<i32> {
    let mut list = Vec::new();
    for _ in 0..length {
        list.push(rand::random::<i32>() % range);
    }
    list
}
