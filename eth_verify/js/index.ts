import { Trie } from "@ethereumjs/trie";
import dotenv from "dotenv";
import { bufferToHex, ecrecover, pubToAddress, rlp, toBuffer, toChecksumAddress } from "ethereumjs-util";
import fs from "fs/promises";
import { keccak256 } from "js-sha3";

interface Header {
  parentHash: string;
  sha3Uncles: string;
  miner: string;
  stateRoot: string;
  transactionsRoot: string;
  receiptsRoot: string;
  logsBloom: string;
  difficulty: string;
  number: string;
  gasLimit: string;
  gasUsed: string;
  timestamp: string;
  totalDifficulty: string;
  extraData: string;
  mixHash: string;
  nonce: string;
  uncles: string[];
  transactions: string[];
  size: string;
}

interface HeaderFromJSON {
  hash: string;
  header: Header;
}

async function main() {
  dotenv.config();
  // Load the fixture block data
  const filePath = "../../inputs/block.json";
  const blockData = JSON.parse(await fs.readFile(filePath, "utf8"));

  // Verify the transactions root
  await verifyBlockTransactionsRoot(blockData);

  // Verify the signature of each transaction
  for (const tx of blockData.transactions) {
    verifyTxSig(tx);
  }

  // Verify the block header & hash
  const {
    parentHash,
    sha3Uncles,
    miner,
    stateRoot,
    transactionsRoot,
    receiptsRoot,
    logsBloom,
    difficulty,
    number,
    gasLimit,
    gasUsed,
    timestamp,
    extraData,
    mixHash,
    nonce,
  } = blockData;
  const blockHeader = [
    parentHash,
    sha3Uncles,
    miner,
    stateRoot,
    transactionsRoot,
    receiptsRoot,
    logsBloom,
    Number(difficulty),
    Number(number),
    Number(gasLimit),
    Number(gasUsed),
    Number(timestamp),
    extraData,
    mixHash,
    nonce,
  ];
  verifyBlock(blockHeader, blockData.hash);
}

main().catch((error) => console.error("An error occurred:", error));

function verifyBlock(blockHeader: any[], hash: string) {
  // compute the hash of the block header
  const encodedBlockHeader = rlp.encode(blockHeader);
  const recomputedHash = "0x" + keccak256(encodedBlockHeader);

  // Compare
  if (recomputedHash !== hash) {
    console.log(`Block header hash: ${hash}`);
    console.log(`Recomputed block header hash: ${recomputedHash}`);
    throw new Error(`❌ Block Header Hash mismatch!`);
  }

  console.log("✅ Block Header hash matches!");
}

async function verifyBlockTransactionsRoot(block: any) {
  // Create a new trie
  const trie = new Trie();

  // Iterate over the transactions in the block
  for (let i = 0; i < block.transactions.length; i++) {
    const tx = block.transactions[i];

    // Use transaction index as the key (RLP encoded)
    const key = rlp.encode(i);

    // RLP encode the transaction itself
    const txData = rlp.encode([
      tx.nonce,
      tx.gasPrice,
      tx.gas,
      tx.to,
      tx.value,
      tx.input,
      tx.v,
      tx.r,
      tx.s,
    ]);

    // Insert the transaction into the trie
    await trie.put(key, txData);
  }

  // Get the root hash of the trie
  const recomputedMptRoot = bufferToHex(Buffer.from(trie.root()));

  // Compare
  if (recomputedMptRoot !== block.transactionsRoot) {
    console.log("Block's MPT Root (transactionsRoot): " + block.transactionsRoot);
    console.log("Recomputed MPT Root: " + recomputedMptRoot);
    throw new Error("❌ MPT Tx Root does not match!");
  }

  console.log("✅ MPT Tx Root matches!");
}

function convertHexVToChainId(hexV: string): number | null {
  // Convert hex to integer
  const v = parseInt(hexV, 16);

  // Check for legacy signatures (pre-EIP-155)
  if (v === 27 || v === 28) {
    return null; // Chain ID not applicable for legacy signatures
  }

  // Calculate chain ID from v
  const chainId = (v - 35) / 2;
  return chainId;
}

function verifyTxSig(tx: any) {
  try {
    // Recover the public key from the signature
    const chainId = convertHexVToChainId(tx.v);
    const hash = toBuffer('0x' + keccak256(rlp.encode([
      tx.nonce,
      tx.gasPrice,
      tx.gas,
      tx.to,
      tx.value,
      chainId,
    ])));
    const v = toBuffer(tx.v);
    const r = toBuffer(tx.r);
    const s = toBuffer(tx.s);
    const pubKey = ecrecover(hash, v, r, s, chainId ?? undefined);

    // Get the address from the public key
    const address = toChecksumAddress(bufferToHex(pubToAddress(pubKey)));

    // Compare
    if (address.toLowerCase() !== tx.from.toLowerCase()) {
      console.log(`Transaction sender: ${tx.from}`);
      console.log(`Recovered address: ${address}`);
      throw new Error(`❌ Invalid signature for transaction: ${parseInt(tx.transactionIndex, 16)}:${tx.hash}`);
    }

    console.log(`✅ Valid signature for transaction: ${parseInt(tx.transactionIndex, 16)}:${tx.hash}`);
  } catch (error) {
    throw new Error(`❌ Error verifying transaction signature: ${(error as Error).message}`);
  }
}