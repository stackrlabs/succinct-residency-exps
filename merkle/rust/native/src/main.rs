use clap::Parser;
use hex;
use merkle::{merkelize};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    num_leaves: i32,
}

fn main() {
    let args = Args::parse();
    let leaves = (0..args.num_leaves)
        .map(|i| i.to_string().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let root = merkelize(leaves);

    println!("Merkle Root: {:?}", hex::encode(root));
}


