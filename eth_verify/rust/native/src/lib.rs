use alloy_primitives::{keccak256, Address, Bloom, Bytes, B256, B64, U256};
use alloy_rlp::{length_of_length, BufMut, Encodable, EMPTY_LIST_CODE, EMPTY_STRING_CODE};
use alloy_trie::{HashBuilder, Nibbles};
use bytes::BytesMut;
use ethereum_types::U256 as U256_ETH;
use hex;
use rlp::RlpStream;
use serde::{Deserialize, Serialize};

// use patricia_trie::memdb::MemoryDB;

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    /// The Keccak 256-bit hash of the parent
    /// block’s header, in its entirety; formally Hp.
    #[serde(rename = "parentHash")]
    pub parent_hash: B256,
    /// The Keccak 256-bit hash of the ommers list portion of this block; formally Ho.
    #[serde(rename = "sha3Uncles")]
    pub ommers_hash: B256,
    /// The 160-bit address to which all fees collected from the successful mining of this block
    /// be transferred; formally Hc.
    #[serde(rename = "miner")]
    pub beneficiary: Address,
    /// The Keccak 256-bit hash of the root node of the state trie, after all transactions are
    /// executed and finalisations applied; formally Hr.
    pub state_root: B256,
    /// The Keccak 256-bit hash of the root node of the trie structure populated with each
    /// transaction in the transactions list portion of the block; formally Ht.
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: B256,
    /// The Keccak 256-bit hash of the root node of the trie structure populated with the receipts
    /// of each transaction in the transactions list portion of the block; formally He.
    pub receipts_root: B256,
    /// The Keccak 256-bit hash of the withdrawals list portion of this block.
    /// <https://eips.ethereum.org/EIPS/eip-4895>
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub withdrawals_root: Option<B256>,
    /// The Bloom filter composed from indexable information (logger address and log topics)
    /// contained in each log entry from the receipt of each transaction in the transactions list;
    /// formally Hb.
    pub logs_bloom: Bloom,
    /// A scalar value corresponding to the difficulty level of this block. This can be calculated
    /// from the previous block’s difficulty level and the timestamp; formally Hd.
    pub difficulty: U256,
    /// A scalar value equal to the number of ancestor blocks. The genesis block has a number of
    /// zero; formally Hi.
    #[serde(with = "alloy_serde::quantity")]
    pub number: u64,
    /// A scalar value equal to the current limit of gas expenditure per block; formally Hl.
    #[serde(with = "alloy_serde::quantity")]
    pub gas_limit: u64,
    /// A scalar value equal to the total gas used in transactions in this block; formally Hg.
    #[serde(with = "alloy_serde::quantity")]
    pub gas_used: u64,
    /// A scalar value equal to the reasonable output of Unix’s time() at this block’s inception;
    /// formally Hs.
    #[serde(with = "alloy_serde::quantity")]
    pub timestamp: u64,
    /// A 256-bit hash which, combined with the
    /// nonce, proves that a sufficient amount of computation has been carried out on this block;
    /// formally Hm.
    pub mix_hash: B256,
    /// A 64-bit value which, combined with the mixhash, proves that a sufficient amount of
    /// computation has been carried out on this block; formally Hn.
    pub nonce: B64,
    /// A scalar representing EIP1559 base fee which can move up or down each block according
    /// to a formula which is a function of gas used in parent block and gas target
    /// (block gas limit divided by elasticity multiplier) of parent block.
    /// The algorithm results in the base fee per gas increasing when blocks are
    /// above the gas target, and decreasing when blocks are below the gas target. The base fee per
    /// gas is burned.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )
    )]
    pub base_fee_per_gas: Option<u64>,
    /// The total amount of blob gas consumed by the transactions within the block, added in
    /// EIP-4844.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )
    )]
    pub blob_gas_used: Option<u64>,
    /// A running total of blob gas consumed in excess of the target, prior to the block. Blocks
    /// with above-target blob gas consumption increase this value, blocks with below-target blob
    /// gas consumption decrease it (bounded at 0). This was added in EIP-4844.
    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            with = "alloy_serde::quantity::opt",
            skip_serializing_if = "Option::is_none"
        )
    )]
    pub excess_blob_gas: Option<u64>,
    /// The hash of the parent beacon block's root is included in execution blocks, as proposed by
    /// EIP-4788.
    ///
    /// This enables trust-minimized access to consensus state, supporting staking pools, bridges,
    /// and more.
    ///
    /// The beacon roots contract handles root storage, enhancing Ethereum's functionalities.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub parent_beacon_block_root: Option<B256>,
    /// The Keccak 256-bit hash of the root node of the trie structure populated with each
    /// [EIP-7685] request in the block body.
    ///
    /// [EIP-7685]: https://eips.ethereum.org/EIPS/eip-7685
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub requests_root: Option<B256>,
    /// An arbitrary byte array containing data relevant to this block. This must be 32 bytes or
    /// fewer; formally Hx.
    pub extra_data: Bytes,
}

impl Header {
    fn header_payload_length(&self) -> usize {
        let mut length = 0;
        length += self.parent_hash.length();
        length += self.ommers_hash.length();
        length += self.beneficiary.length();
        length += self.state_root.length();
        length += self.transactions_root.length();
        length += self.receipts_root.length();
        length += self.logs_bloom.length();
        length += self.difficulty.length();
        length += U256::from(self.number).length();
        length += U256::from(self.gas_limit).length();
        length += U256::from(self.gas_used).length();
        length += self.timestamp.length();
        length += self.extra_data.length();
        length += self.mix_hash.length();
        length += self.nonce.length();

        if let Some(base_fee) = self.base_fee_per_gas {
            length += U256::from(base_fee).length();
        } else if self.withdrawals_root.is_some()
            || self.blob_gas_used.is_some()
            || self.excess_blob_gas.is_some()
            || self.parent_beacon_block_root.is_some()
        {
            length += 1; // EMPTY LIST CODE
        }

        if let Some(root) = self.withdrawals_root {
            length += root.length();
        } else if self.blob_gas_used.is_some()
            || self.excess_blob_gas.is_some()
            || self.parent_beacon_block_root.is_some()
        {
            length += 1; // EMPTY STRING CODE
        }

        if let Some(blob_gas_used) = self.blob_gas_used {
            length += U256::from(blob_gas_used).length();
        } else if self.excess_blob_gas.is_some() || self.parent_beacon_block_root.is_some() {
            length += 1; // EMPTY LIST CODE
        }

        if let Some(excess_blob_gas) = self.excess_blob_gas {
            length += U256::from(excess_blob_gas).length();
        } else if self.parent_beacon_block_root.is_some() {
            length += 1; // EMPTY LIST CODE
        }

        // Encode parent beacon block root length.
        if let Some(parent_beacon_block_root) = self.parent_beacon_block_root {
            length += parent_beacon_block_root.length();
        }

        // Encode requests root length.
        //
        // If new fields are added, the above pattern will
        // need to be repeated and placeholder length added. Otherwise, it's impossible to
        // tell _which_ fields are missing. This is mainly relevant for contrived cases
        // where a header is created at random, for example:
        //  * A header is created with a withdrawals root, but no base fee. Shanghai blocks are
        //    post-London, so this is technically not valid. However, a tool like proptest would
        //    generate a block like this.
        if let Some(requests_root) = self.requests_root {
            length += requests_root.length();
        }

        length
    }
}

impl Encodable for Header {
    fn encode(&self, out: &mut dyn BufMut) {
        let list_header = alloy_rlp::Header {
            list: true,
            payload_length: self.header_payload_length(),
        };
        list_header.encode(out);
        self.parent_hash.encode(out);
        self.ommers_hash.encode(out);
        self.beneficiary.encode(out);
        self.state_root.encode(out);
        self.transactions_root.encode(out);
        self.receipts_root.encode(out);
        self.logs_bloom.encode(out);
        self.difficulty.encode(out);
        U256::from(self.number).encode(out);
        U256::from(self.gas_limit).encode(out);
        U256::from(self.gas_used).encode(out);
        self.timestamp.encode(out);
        self.extra_data.encode(out);
        self.mix_hash.encode(out);
        self.nonce.encode(out);

        // Encode base fee. Put empty list if base fee is missing,
        // but withdrawals root is present.
        if let Some(ref base_fee) = self.base_fee_per_gas {
            U256::from(*base_fee).encode(out);
        } else if self.withdrawals_root.is_some()
            || self.blob_gas_used.is_some()
            || self.excess_blob_gas.is_some()
            || self.parent_beacon_block_root.is_some()
        {
            out.put_u8(EMPTY_LIST_CODE);
        }

        // Encode withdrawals root. Put empty string if withdrawals root is missing,
        // but blob gas used is present.
        if let Some(ref root) = self.withdrawals_root {
            root.encode(out);
        } else if self.blob_gas_used.is_some()
            || self.excess_blob_gas.is_some()
            || self.parent_beacon_block_root.is_some()
        {
            out.put_u8(EMPTY_STRING_CODE);
        }

        // Encode blob gas used. Put empty list if blob gas used is missing,
        // but excess blob gas is present.
        if let Some(ref blob_gas_used) = self.blob_gas_used {
            U256::from(*blob_gas_used).encode(out);
        } else if self.excess_blob_gas.is_some() || self.parent_beacon_block_root.is_some() {
            out.put_u8(EMPTY_LIST_CODE);
        }

        // Encode excess blob gas. Put empty list if excess blob gas is missing,
        // but parent beacon block root is present.
        if let Some(ref excess_blob_gas) = self.excess_blob_gas {
            U256::from(*excess_blob_gas).encode(out);
        } else if self.parent_beacon_block_root.is_some() {
            out.put_u8(EMPTY_LIST_CODE);
        }

        // Encode parent beacon block root.
        if let Some(ref parent_beacon_block_root) = self.parent_beacon_block_root {
            parent_beacon_block_root.encode(out);
        }

        // Encode requests root.
        //
        // If new fields are added, the above pattern will need to
        // be repeated and placeholders added. Otherwise, it's impossible to tell _which_
        // fields are missing. This is mainly relevant for contrived cases where a header is
        // created at random, for example:
        //  * A header is created with a withdrawals root, but no base fee. Shanghai blocks are
        //    post-London, so this is technically not valid. However, a tool like proptest would
        //    generate a block like this.
        if let Some(ref requests_root) = self.requests_root {
            requests_root.encode(out);
        }
    }

    fn length(&self) -> usize {
        let mut length = 0;
        length += self.header_payload_length();
        length += length_of_length(length);
        length
    }
}

#[no_mangle]
pub fn verify_block_hash(header: Header, expected_hash: B256) -> bool {
    let recomputed_hash = keccak256(alloy_rlp::encode(header));
    assert_eq!(recomputed_hash, expected_hash);
    recomputed_hash == expected_hash
}



/// Adjust the index of an item for rlp encoding.
pub const fn adjust_index_for_rlp(i: usize, len: usize) -> usize {
    if i > 0x7f {
        i
    } else if i == 0x7f || i + 1 == len {
        0
    } else {
        i + 1
    }
}

// Struct representing the transaction data
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    nonce: String,
    gasPrice: String,
    gas: String,
    to: String,
    value: String,
    input: String,
    v: String,
    r: String,
    s: String,
}

// Struct representing the block containing transactions
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    #[serde(rename = "transactionsRoot")]
    transactions_root: String,
    transactions: Vec<Transaction>,
}

pub fn rlp_encode_transaction(tx: &Transaction) -> BytesMut {
    let mut rlp_stream = RlpStream::new_list(9);

    // Handle potential large values by using BigUint
    // Convert nonce, gasPrice, gasLimit, value, and v to U256 for large number support
    let nonce = U256_ETH::from_str_radix(&tx.nonce[2..], 16).unwrap();
    let gas_price = U256_ETH::from_str_radix(&tx.gasPrice[2..], 16).unwrap();
    let gas_limit = U256_ETH::from_str_radix(&tx.gas[2..], 16).unwrap(); // Corrected to "gasLimit"
    let to = hex::decode(&tx.to[2..]).unwrap();
    let value = U256_ETH::from_str_radix(&tx.value[2..], 16).unwrap();
    let input = hex::decode(&tx.input[2..]).unwrap();
    let v = U256_ETH::from_str_radix(&tx.v[2..], 16).unwrap();
    let r = hex::decode(&tx.r[2..]).unwrap();
    let s = hex::decode(&tx.s[2..]).unwrap();

    // Append nonce, gasPrice, gas, value, v (big numbers)
    // Append the fields in the correct order
    println!("nonce: {:?}", nonce);
    println!("gas_price: {:?}", gas_price);
    println!("gas_limit: {:?}", gas_limit);
    println!("to: {:?}", to);
    println!("value: {:?}", value);
    println!("input: {:?}", input);
    println!("v: {:?}", v);
    println!("r: {:?}", r);
    println!("s: {:?}", s);

    rlp_stream.append(&nonce); // nonce
    rlp_stream.append(&gas_price); // gasPrice
    rlp_stream.append(&gas_limit); // gasLimit
    rlp_stream.append(&to); // to (address)
    rlp_stream.append(&value); // value
    rlp_stream.append(&input); // input (data)
    rlp_stream.append(&v); // v (signature recovery)
    rlp_stream.append(&r); // r (signature part)
    rlp_stream.append(&s); // s (signature part)s

    // Return RLP-encoded transaction
    rlp_stream.out()
}

// Function to calculate the MPT root of transactions
pub async fn calculate_mpt_root(block: Block) -> bool {
    // Initialize a memory-backed database for the trie
    // let mut memdb = MemoryDB::<Keccak256Hasher>::default();
    // let mut root = Default::default();
    // let mut trie = TrieDBMut::new(&mut memdb, &mut root);

    let mut hb = HashBuilder::default();

    // Iterate over transactions, RLP encode and insert into the trie
    for (i, tx) in block.transactions.iter().enumerate().rev() {
        println!("i: {:?}", i);
        let rlp_encoded_tx = rlp_encode_transaction(tx);
        println!("rlp_encoded_tx: {:?}", hex::encode(&rlp_encoded_tx));
        // Encode the index as the key (RLP encoded)
        let index = i as u64;
        let index_buffer = alloy_rlp::encode_fixed_size(&index);

        // let rlp_encoded_index = rlp::encode(&i);
        let key = Nibbles::unpack(&index_buffer);
        println!("key: {:?}", hex::encode(key.as_ref()));

        // Insert the transaction into the trie
        hb.add_leaf(key, &rlp_encoded_tx);
        // trie.insert(&rlp_encoded_index, &rlp_encoded_tx)?;
    }

    // Calculate the root of the trie
    // let trie_root= trie.root()?;
    let trie_root = hb.root();
    // Print the calculated root
    println!("Calculated MPT Root: 0x{}", hex::encode(trie_root));

    // Compare with the block's official transactionsRoot
    println!("Block's transactionsRoot: {}", block.transactions_root);

    // Check if the roots match
    if hex::encode(trie_root) == block.transactions_root[2..] {
        return true;
    } else {
        return false;
    }
}