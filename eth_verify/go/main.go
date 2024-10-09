package main

import (
	"encoding/json"
	"fmt"
	"log"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/ethereum/go-ethereum/trie"
	"golang.org/x/crypto/sha3"
)

type BlockFromJSON struct {
	Number int64       `json:"number"`
	Hash   common.Hash `json:"hash"`
}

func main() {
	blockJSON, err := os.ReadFile("../../inputs/block.json")
	if err != nil {
		log.Fatalf("Failed to read block JSON from file: %v", err)
	}
	var block struct {
		Transactions types.Transactions `json:"transactions"`
		Hash         common.Hash        `json:"hash"`
	}
	err = json.Unmarshal(blockJSON, &block)
	if err != nil {
		log.Fatalf("Failed to unmarshal block JSON: %v", err)
	}
	transactions := block.Transactions

	var header types.Header
	headerJSON, err := os.ReadFile("../../inputs/block.json")
	if err != nil {
		log.Fatalf("Failed to read header JSON from file: %v", err)
	}
	err = json.Unmarshal(headerJSON, &header)
	if err != nil {
		log.Fatalf("Failed to unmarshal header JSON: %v", err)
	}

	// Create a new trie
	tr, err := trie.New(trie.TrieID(common.Hash{}), nil)
	if err != nil {
		log.Fatalf("Failed to create trie: %v", err)
	}

	// Iterate over the transactions in the block
	for i, tx := range transactions {
		// Get the transaction index as a key (RLP encoded)
		key, err := rlp.EncodeToBytes(big.NewInt(int64(i)))
		if err != nil {
			log.Fatalf("Failed to encode transaction index: %v", err)
		}

		// RLP encode the transaction itself
		txData, err := rlp.EncodeToBytes(tx)
		if err != nil {
			log.Fatalf("Failed to encode transaction: %v", err)
		}

		// Insert the transaction into the trie
		err = tr.Update(key, txData)
		if err != nil {
			log.Fatalf("Failed to update trie: %v", err)
		}
	}

	// Calculate the root hash of the trie
	mptRoot := tr.Hash()

	// Output the calculated MPT root
	fmt.Printf("Calculated MPT Root: %s\n", mptRoot.Hex())

	// Compare with the block's official TxHash root (should match)
	fmt.Printf("Block's MPT Root (TxHash): %v\n", header.TxHash)

	if mptRoot != header.TxHash {
		log.Fatalf("MPT Root does not match block's TxHash")
		return
	}

	result := verifyBlock(header, block.Hash)
	if !result {
		log.Fatalf("Block failed to verify")
		return
	}
	fmt.Printf("Block successfuly verified\n")
}

func verifyBlock(header types.Header, hash common.Hash) bool {
	hw := sha3.NewLegacyKeccak256()
	rlp.Encode(hw, &header)
	recomputedHash := common.Hash(hw.Sum(nil))
	fmt.Printf("Recomputed hash: %s\n", recomputedHash.Hex())
	return recomputedHash == hash
}
