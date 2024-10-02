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

  if (leaves.length === 1) {
    return sha256Hash(leaves[0]);
  }

  let level: Buffer[] = leaves.map((leaf) => sha256Hash(leaf));

  while (level.length > 1) {
    const nextLevel: Buffer[] = [];
    for (let i = 0; i < level.length; i += 2) {
      if (i + 1 < level.length) {
        const combined = Buffer.concat([level[i], level[i + 1]]);
        nextLevel.push(sha256Hash(combined));
      } else {
        nextLevel.push(level[i]);
      }
    }
    level = nextLevel;
  }

  return level[0];
}

function sha256Hash(data: Buffer): Buffer {
  return crypto.createHash("sha256").update(data).digest();
}
