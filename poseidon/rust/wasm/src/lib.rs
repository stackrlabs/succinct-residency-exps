use ark_bn254::Fr;
use ark_std::str::FromStr;
use poseidon_ark::Poseidon;

#[no_mangle]
pub fn poseidon_hash(arr_len: u32) -> u32 {
    let mut input_arr: Vec<Fr> = Vec::with_capacity(arr_len as usize);
    for i in 0..arr_len as usize {
        input_arr.push(Fr::from_str(&i.to_string()).unwrap());
    }
    let poseidon = Poseidon::new();
    let hash = poseidon.hash(input_arr.clone()).unwrap();
    println!("Array Length: {:?}", arr_len);
    println!("Hash: {:?}", hash);
    1
}

