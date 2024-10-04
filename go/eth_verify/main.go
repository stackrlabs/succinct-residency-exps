package main

import (
	"encoding/json"
	"fmt"
	"log"
	"os"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/rlp"
	"golang.org/x/crypto/sha3"
)

type HeaderFromJSON struct {
	Hash   common.Hash  `json:"hash"`
	Header types.Header `json:"header"`
}

func main() {
	filePath := "../../inputs/block_data.json"
	fileContent, err := os.ReadFile(filePath)
	if err != nil {
		log.Fatalf("Failed to read the file: %v", err)
	}
	jsonString := string(fileContent)

	var headerFromJSON HeaderFromJSON
	err = json.Unmarshal([]byte(jsonString), &headerFromJSON)
	if err != nil {
		log.Fatalf("Failed to unmarshal JSON: %v", err)
	}

	hw := sha3.NewLegacyKeccak256()
	rlp.Encode(hw, &headerFromJSON.Header)
	hash := hw.Sum(nil)

	recomputedHash := common.Hash(hash)

	if recomputedHash != headerFromJSON.Hash {
		log.Fatalf("Hash mismatch: recomputed %v, block hash %v", recomputedHash, headerFromJSON.Hash)
	}

	fmt.Printf("Recomputed hash: %v\n", recomputedHash.Hex())
}
