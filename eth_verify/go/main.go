package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"math/big"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/ethclient"
	"github.com/ethereum/go-ethereum/rlp"
	"github.com/ethereum/go-ethereum/trie"
	"github.com/joho/godotenv"
	"golang.org/x/crypto/sha3"
)

type BlockFromJSON struct {
	Number int64       `json:"number"`
	Hash   common.Hash `json:"hash"`
}

func main() {
	// Connect to a local Geth node or a remote Ethereum node
	godotenv.Load()

	nodeURL := os.Getenv("ETH_NODE_URL")
	if nodeURL == "" {
		log.Fatalf("ETH_NODE_URL not set")
	}
	client, err := ethclient.Dial(nodeURL)
	if err != nil {
		log.Fatalf("Failed to connect to the Ethereum client: %v", err)
	}

	var blockData BlockFromJSON
	blockNumberJSON, err := os.ReadFile("../../inputs/block.json")
	if err != nil {
		log.Fatalf("Failed to read block number JSON from file: %v", err)
	}
	err = json.Unmarshal(blockNumberJSON, &blockData)
	if err != nil {
		log.Fatalf("Failed to unmarshal block number JSON: %v", err)
	}
	blockNumber := big.NewInt(blockData.Number)

	// Fetch the block by number
	block, err := client.BlockByNumber(context.Background(), blockNumber)
	if err != nil {
		log.Fatalf("Failed to retrieve block: %v", err)
	}

	// Create a new trie
	tr, err := trie.New(trie.TrieID(common.Hash{}), nil)
	if err != nil {
		log.Fatalf("Failed to create trie: %v", err)
	}

	// Iterate over the transactions in the block
	for i, tx := range block.Transactions() {
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
	fmt.Printf("Block's MPT Root (TxHash): %v\n", block.Header().TxHash)

	if mptRoot != block.Header().TxHash {
		log.Fatalf("MPT Root does not match block's TxHash")
		return
	}

	result := verifyBlock(*block.Header(), blockData.Hash)
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
