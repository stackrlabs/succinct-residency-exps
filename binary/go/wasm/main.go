package main

import (
	"unsafe"
)

//go:wasm-module binarySearch
//export binarySearch
func BinarySearch(dataPtr *int32, count int32, target int32) int32 {
	list32 := readList(dataPtr, count)
	list := make([]int, len(list32))
	for i, v := range list32 {
		list[i] = int(v)
	}
	return int32(BinarySearchImpl(list, int(target)))
}

func BinarySearchImpl(list []int, target int) int {
	left := 0
	right := len(list) - 1

	for left <= right {
		mid := (left + right) / 2
		if list[mid] == target {
			return mid
		}
		if list[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return -1
}

func readList(dataPtr *int32, count int32) []int32 {
	println("readList")
	// Cast the pointer to uintptr for arithmetic
	baseAddr := uintptr(unsafe.Pointer(dataPtr))
	result := make([]byte, count*4)

	for i := 0; i < len(result); i++ {
		result[i] = *(*byte)(unsafe.Pointer(baseAddr + uintptr(i)))
	}

	// Convert the byte slice back to a slice of int32
	var intSlice []int32
	for i := 0; i < len(result); i += 4 {
		intSlice = append(intSlice, int32(result[i])|int32(result[i+1])<<8|int32(result[i+2])<<16|int32(result[i+3])<<24)
	}

	return intSlice
}
