package main

import (
	"crypto/sha256"
	"fmt"
	"math"
	"os"
	"strconv"

	"github.com/ethereum/go-ethereum/common"
)

func main() {
	// Parse command-line arguments
	if len(os.Args) != 2 {
		fmt.Println("Usage: go run main.go <num-leaves>")
		return
	}

	numLeaves, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Invalid number:", os.Args[1])
		return
	}

	leaves := make([][]byte, numLeaves)
	for i := 0; i < numLeaves; i++ {
		leaves[i] = []byte(fmt.Sprintf("%d", i))
	}

	root := merkelize(leaves)
	fmt.Printf("Merkle root: %s\n", root.Hex())
}

func merkelize(leaves [][]byte) common.Hash {
	if len(leaves) == 0 {
		return common.Hash{}
	}

	// Ensure the number of leaves is a power of two
	targetSize := 1 << uint(math.Ceil(math.Log2(float64(len(leaves)))))
	for len(leaves) < targetSize {
		leaves = append(leaves, []byte{})
	}

	var level []common.Hash

	for _, leaf := range leaves {
		level = append(level, sha256Hash(leaf))
	}

	for len(level) > 1 {
		nextLevel := make([]common.Hash, (len(level)+1)/2)
		for i := 0; i < len(level); i += 2 {
			if i+1 < len(level) {
				combined := append(level[i].Bytes(), level[i+1].Bytes()...)
				nextLevel[i/2] = sha256Hash(combined)
			} else {
				nextLevel[i/2] = level[i]
			}
		}
		level = nextLevel
	}

	return level[0]
}

func sha256Hash(data []byte) common.Hash {
	hash := sha256.Sum256(data)
	return common.BytesToHash(hash[:])
}
