use clap::Parser;
use hex;
use sha2::{Digest, Sha256};

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

fn merkelize(leaves: Vec<Vec<u8>>) -> [u8; 32] {
    if leaves.is_empty() {
        return [0; 32];
    }

    // Ensure the number of leaves is a power of two
    let target_size = 2_usize.pow((leaves.len() as f64).log2().ceil() as u32);
    let mut padded_leaves = leaves;
    while padded_leaves.len() < target_size {
        padded_leaves.push(Vec::new());
    }

    let mut level: Vec<[u8; 32]> = padded_leaves.iter().map(|leaf| sha256_hash(leaf)).collect();

    while level.len() > 1 {
        let mut next_level = Vec::with_capacity((level.len() + 1) / 2);
        for chunk in level.chunks(2) {
            if chunk.len() == 2 {
                let combined = [&chunk[0][..], &chunk[1][..]].concat();
                next_level.push(sha256_hash(&combined));
            } else {
                next_level.push(chunk[0]);
            }
        }
        level = next_level;
    }

    level[0]
}

fn sha256_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}