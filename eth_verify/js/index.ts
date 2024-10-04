import { keccak256 } from 'js-sha3';
import fs from 'fs/promises';
import { rlp, toBuffer } from 'ethereumjs-util';

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
    const fileContent = await fs.readFile(filePath, 'utf-8');
    const jsonString = fileContent.toString();

    const headerFromJSON: HeaderFromJSON = JSON.parse(jsonString);
    const { parentHash, sha3Uncles, miner, stateRoot, transactionsRoot, receiptsRoot, logsBloom, difficulty, number, gasLimit, gasUsed, timestamp, extraData, mixHash, nonce } = headerFromJSON.header;
    const blockHeader = [
        toBuffer(parentHash),
        toBuffer(sha3Uncles),
        toBuffer(miner),
        toBuffer(stateRoot),
        toBuffer(transactionsRoot),
        toBuffer(receiptsRoot),
        toBuffer(logsBloom),
        Number(difficulty),
        Number(number),
        Number(gasLimit),
        Number(gasUsed),
        Number(timestamp),
        toBuffer(extraData),
        toBuffer(mixHash),
        toBuffer(nonce)
    ];
    verifyBlockHash(blockHeader, headerFromJSON.hash);
}

main().catch(error => console.error("An error occurred:", error));

function verifyBlockHash(blockHeader: any[], compareHash: string) {
    const encodedBlockHeader = rlp.encode(blockHeader);
    const recomputedHash = "0x" + keccak256(encodedBlockHeader); 

    if (recomputedHash !== compareHash) {
        throw new Error(`Hash meismatch: recomputed ${recomputedHash}, block hash ${compareHash}`);
    }

    console.log(`Recomputed hash: ${recomputedHash}`);
}