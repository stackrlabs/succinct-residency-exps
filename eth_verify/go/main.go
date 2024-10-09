package main

import (
	"encoding/json"
	"fmt"
	"log"
	"math/big"
	"os"
	"strconv"

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

type TransactionFromJSON struct {
	types.Transaction
	From             common.Address `json:"from"`
	TransactionIndex uint64         `json:"transactionIndex"`
}

func (tx *TransactionFromJSON) UnmarshalJSON(data []byte) error {
	// Unmarshal custom fields
	var txJSON struct {
		From             string `json:"from"`
		TransactionIndex string `json:"transactionIndex"`
	}
	err := json.Unmarshal(data, &txJSON)
	if err != nil {
		return err
	}

	// Unmarshal the types.Transaction fields
	err = json.Unmarshal(data, &tx.Transaction)
	if err != nil {
		return err
	}

	// Parse the custom fields
	tx.From = common.HexToAddress(txJSON.From)
	tx.TransactionIndex, _ = strconv.ParseUint(txJSON.TransactionIndex, 16, 64)
	return nil
}

func main() {
	// Load the block.json fixture into block and header vars
	blockJSON, err := os.ReadFile("../../inputs/block.json")
	if err != nil {
		log.Fatalf("Failed to read block JSON from file: %v", err)
	}
	var block struct {
		Transactions []*TransactionFromJSON `json:"transactions"`
		Hash         common.Hash            `json:"hash"`
	}
	err = json.Unmarshal(blockJSON, &block)
	if err != nil {
		log.Fatalf("Failed to unmarshal block JSON: %v", err)
	}
	var header types.Header
	headerJSON, err := os.ReadFile("../../inputs/block.json")
	if err != nil {
		log.Fatalf("Failed to read header JSON from file: %v", err)
	}
	err = json.Unmarshal(headerJSON, &header)
	if err != nil {
		log.Fatalf("Failed to unmarshal header JSON: %v", err)
	}

	// Verify the txs root
	verifyBlockTransactionsRoot(header, block.Transactions)

	// Verify the signature of each tx
	for i := range block.Transactions {
		verifyTxSig(block.Transactions[i])
	}

	// Verify the block header & hash
	verifyBlock(header, block.Hash)
}

func verifyBlock(header types.Header, hash common.Hash) {
	hw := sha3.NewLegacyKeccak256()
	rlp.Encode(hw, &header)
	recomputedHash := common.Hash(hw.Sum(nil))

	// Compare
	if recomputedHash != hash {
		fmt.Printf("Block header hash: %s\n", hash.Hex())
		fmt.Printf("Recomputed block header hash: %s\n", recomputedHash.Hex())
		log.Fatalf("❌ Block Header Hash does not match!")
		return
	}

	fmt.Println("✅ Block Header Hash matches!")
}

func verifyBlockTransactionsRoot(header types.Header, transactions []*TransactionFromJSON) {
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
		txData, err := rlp.EncodeToBytes(&tx.Transaction)
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

	// Compare
	if mptRoot != header.TxHash {
		fmt.Printf("Block's MPT Root (transactionsRoot): %s\n", header.TxHash.Hex())
		fmt.Printf("Calculated MPT Root: %s\n", mptRoot.Hex())
		log.Fatal("❌ MPT Tx Root does not match!")
	}

	fmt.Println("✅ MPT Tx Root matches!")
}

func verifyTxSig(tx *TransactionFromJSON) {
	// Recover the address from the signature
	signer := types.LatestSignerForChainID(tx.ChainId())
	addr, err := types.Sender(signer, &tx.Transaction)
	if err != nil {
		log.Fatalf("Failed to recover sender address: %v", err)
	}

	// Compare
	if addr != tx.From {
		fmt.Printf("Transaction sender: %s\n", tx.From.Hex())
		fmt.Printf("Recovered address: %s\n", addr.Hex())
		log.Fatalf("❌ Invalid signature for transaction: %d:%s", tx.TransactionIndex, tx.Hash().Hex())
	}

	fmt.Printf("✅ Valid signature for transaction: %d:%s\n", tx.TransactionIndex, tx.Hash().Hex())
}
