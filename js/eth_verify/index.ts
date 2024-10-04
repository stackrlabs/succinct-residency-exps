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
    console.log(headerFromJSON.header)
    const blockHeader = [
        toBuffer(headerFromJSON.header.parentHash),
        toBuffer(headerFromJSON.header.sha3Uncles),
        toBuffer(headerFromJSON.header.miner),
        toBuffer(headerFromJSON.header.stateRoot),
        toBuffer(headerFromJSON.header.transactionsRoot),
        toBuffer(headerFromJSON.header.receiptsRoot),
        toBuffer(headerFromJSON.header.logsBloom),
        toBuffer(parseInt(headerFromJSON.header.difficulty.slice(2), 16)),
        toBuffer(parseInt(headerFromJSON.header.number.slice(2), 16)),
        toBuffer(parseInt(headerFromJSON.header.gasLimit.slice(2), 16)),
        toBuffer(parseInt(headerFromJSON.header.gasUsed.slice(2), 16)),
        toBuffer(parseInt(headerFromJSON.header.timestamp.slice(2), 16)),
        toBuffer(headerFromJSON.header.extraData),
        toBuffer(headerFromJSON.header.mixHash),
        toBuffer(headerFromJSON.header.nonce)
    ];

    const encodedBlockHeader = rlp.encode(blockHeader);
    const recomputedHash = "0x" + keccak256(encodedBlockHeader); 

    if (recomputedHash !== headerFromJSON.hash) {
        throw new Error(`Hash mismatch: recomputed ${recomputedHash}, block hash ${headerFromJSON.hash}`);
    }

    console.log(`Recomputed hash: ${recomputedHash}`);
}

main().catch(error => console.error("An error occurred:", error));
