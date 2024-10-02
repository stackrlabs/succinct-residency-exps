use clap::Parser;
use rs_merkle::{algorithms::Sha256, MerkleTree, Hasher};
use hex;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    num_leaves: i32,
}

fn main() {
    let args = Args::parse();
    let leaves = (0..args.num_leaves)
        .map(|i| Sha256::hash(i.to_string().as_bytes()))
        .collect::<Vec<_>>();
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    let root = merkle_tree.root().unwrap();
    println!("Merkle Root: {:?}", hex::encode(root));
}
