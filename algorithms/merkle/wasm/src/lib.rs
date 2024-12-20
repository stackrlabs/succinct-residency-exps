use sha2::{Digest, Sha256};
use hex;

pub fn merkelize_impl(leaves: Vec<[u8; 32]>) -> [u8; 32] {
    if leaves.is_empty() {
        return [0; 32];
    }

    let mut curr_level: Vec<[u8; 32]> = leaves.iter().map(|leaf| sha256_hash(leaf)).collect();
    let mut next_level: Vec<[u8; 32]> = Vec::with_capacity((curr_level.len() + 1) / 2);
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

#[no_mangle]
pub fn merkelize(leaves_base_2: i32) -> u32 {
    let total_leaves = 2_usize.pow(leaves_base_2 as u32);
    let leaves = vec![[0u8; 32]; total_leaves];
    let root = merkelize_impl(leaves);
    println!("Merkle root: {:?}", hex::encode(root));
    1
}

fn sha256_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}
