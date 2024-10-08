use wasm::binary_search;
use clap::Parser;
use rand;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    number: i32,
    #[arg(short, long)]
    length: i32,
}

fn main() {
    let args = Args::parse();
    let mut list = generate_random_list(args.length, args.length);
    list.sort();
    println!("List: {:?}", list);
    let found = binary_search(list, args.number);
    println!("Element found?: {}", found);
}

fn generate_random_list(length: i32, range: i32) -> Vec<i32> {
    let mut list = Vec::new();
    for _ in 0..length {
        list.push(rand::random::<i32>() % range);
    }
    list
}

