use sha2::{Digest, Sha256};

pub fn merkelize(leaves: Vec<Vec<u8>>) -> [u8; 32] {
    if leaves.is_empty() {
        return [0; 32];
    }

    // Ensure the number of leaves is a power of two
    let target_size = 2_usize.pow((leaves.len() as f64).log2().ceil() as u32);
    let mut padded_leaves = leaves;
    while padded_leaves.len() < target_size {
        padded_leaves.push(Vec::new());
    }

    let mut curr_level: Vec<[u8; 32]> = padded_leaves.iter().map(|leaf| sha256_hash(leaf)).collect();
    let mut next_level = Vec::with_capacity((curr_level.len() + 1) / 2);
    while curr_level.len() > 1 {
        for chunk in curr_level.chunks(2) {
            if chunk.len() == 2 {
                let combined = [&chunk[0][..], &chunk[1][..]].concat();
                next_level.push(sha256_hash(&combined));
            } else {
                next_level.push(chunk[0]);
            }
        }
        std::mem::swap(&mut curr_level, &mut next_level);
        next_level.clear();
    }

    curr_level[0]
}

fn sha256_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
