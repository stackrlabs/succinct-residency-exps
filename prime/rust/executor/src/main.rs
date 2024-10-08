use wasm::is_prime;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    number: i32,
}

fn main() {
    let args = Args::parse();
    let is_prime = is_prime(args.number);
    println!("Is prime: {}", is_prime);
}
