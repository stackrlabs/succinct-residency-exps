use sha3::{Digest, Keccak256};
use hex;

#[no_mangle]
pub fn keccak_n(n: u32) -> u32 {
    const INPUT: &[u8] = b"hello world";
    const EXPECTED_HASH: &str = "47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad";
    for _ in 1..=n {
        let mut hasher = Keccak256::new();
        hasher.update(INPUT);
        let result = hasher.finalize();
        assert_eq!(hex::encode(result), EXPECTED_HASH);
    }
    1
}
