import { rlp } from "ethereumjs-util";
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
  const filePath = "../../inputs/block_data.json";
  const fileContent = await fs.readFile(filePath, "utf-8");
  const jsonString = fileContent.toString();

  const headerFromJSON: HeaderFromJSON = JSON.parse(jsonString);
  const { header, hash } = headerFromJSON;
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
  } = header;

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
  verifyBlock(blockHeader, hash);
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

  console.log(`✅ Block Verified\nRecomputed hash: ${recomputedHash}`);
}
