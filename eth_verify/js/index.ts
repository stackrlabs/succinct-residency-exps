import { Trie } from "@ethereumjs/trie";
import dotenv from "dotenv";
import { rlp } from "ethereumjs-util";
import fs from "fs/promises";
import { keccak256 } from "js-sha3";
import Web3 from "web3";

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
  const filePath = "../../inputs/block.json";
  const blockData = JSON.parse(await fs.readFile(filePath, "utf8"));

  const mptRoot = await calculateMptRoot(blockData);
  console.log("Calculated MPT Root: " + mptRoot);
  console.log("Block's MPT Root (TxHash): " + blockData.transactionsRoot);
  if (mptRoot !== blockData.transactionsRoot) {
    throw new Error("MPT Root does not match");
  }
  console.log("Block's MPT Root (TxHash): " + blockData.transactionsRoot);

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
  const encodedBlockHeader = rlp.encode(blockHeader);
  const recomputedHash = "0x" + keccak256(encodedBlockHeader);

  if (recomputedHash !== hash) {
    throw new Error(
      `❌ Hash mismatch: recomputed ${recomputedHash}, block hash ${hash}`
    );
  }

  console.log(`Recomputed hash: ${recomputedHash}`);
}

async function calculateMptRoot(block: any) {
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
  const mptRoot = trie.root();
  return "0x" + Buffer.from(mptRoot).toString("hex");
}
