function binarySearch(arr: number[], target: number): number {
    let left = 0;
    let right = arr.length - 1;

    while (left <= right) {
        const mid = Math.floor((left + right) / 2);
        if (arr[mid] === target) {
            return mid;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

function generateSortedRandomList(length: number): number[] {
    const list = Array.from({ length }, () => Math.floor(Math.random() * length));
    return list.sort((a, b) => a - b);
}

function searchRandomList(target: number, length: number): void {
    const randomList = generateSortedRandomList(length);
    console.log("Generated list:", randomList);

    const result = binarySearch(randomList, target);

    if (result !== -1) {
        console.log(`Target ${target} found at index ${result}`);
    } else {
        console.log(`Target ${target} not found in the list`);
    }
}

// Example usage
const targetNumber = parseInt(prompt("Enter the target number to search:") || "");
const listLength = parseInt(prompt("Enter the length of the list to search:") || "");

if (isNaN(targetNumber) || isNaN(listLength)) {
    throw new Error("Please provide valid numbers for target and list length as command-line arguments.");
}
searchRandomList(targetNumber, listLength);
