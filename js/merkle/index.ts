import crypto from "crypto";

if (process.argv.length !== 3) {
  console.log("Usage: bun run index.ts <num-leaves>");
  process.exit(1);
}

const numLeaves = parseInt(process.argv[2]);
if (isNaN(numLeaves)) {
  console.log("Invalid number:", process.argv[2]);
  process.exit(1);
}

const leaves: Buffer[] = [];
for (let i = 0; i < numLeaves; i++) {
  leaves.push(Buffer.from(i.toString()));
}

const root = merkelize(leaves);
console.log(`Merkle root: ${root.toString("hex")}`);

function merkelize(leaves: Buffer[]): Buffer {
  if (leaves.length === 0) {
    return Buffer.alloc(32);
  }
  // Ensure the number of leaves is a power of two
  const targetSize = Math.pow(2, Math.ceil(Math.log2(leaves.length)));
  while (leaves.length < targetSize) {
    leaves.push(Buffer.alloc(0));
  }

  let level: Buffer[] = leaves.map((leaf) => sha256Hash(leaf));

  while (level.length > 1) {
    const nextLevel: Buffer[] = [];
    for (let i = 0; i < level.length; i += 2) {
      const combined = Buffer.concat([level[i], level[i + 1]]);
      nextLevel.push(sha256Hash(combined));
    }
    level = nextLevel;
  }

  return level[0];
}

function sha256Hash(data: Buffer): Buffer {
  return crypto.createHash("sha256").update(data).digest();
}
