/**
 * Reducer Method, that needs to be called
 * per array to map array items to unique
 * key-value pairs, incrementing counts for
 * each item.
 *
 * @param {Record<string, number>} acc
 * @param {[number, string]} item
 */
const reduceItemCount = (acc, item) => {
    if (!item || !item.length) {
        return acc;
    }
    const [count, key] = item;
    acc[key] = acc[key] ? acc[key] + count : count;
    return acc;
}

/**
 * Complexitities:
 * - Time: O(n)
 * - Space: O(n)
 *
 * @param {Array<[number, string]>} arr1
 * @param {Array<[number, string]>} arr2
 */
function updateInventory(arr1, arr2) {
    if (!arr1 && !arr2) {
        return [];
    }
    if (!arr1 && arr2) {
        return arr2;
    }
    if (arr1 && !arr2) {
        return arr1;
    }

    // O(m)
    let map = arr1.reduce(reduceItemCount, {});
    // O(n)
    map = arr2.reduce(reduceItemCount, map);

    // O(p), where p >= m | n
    return Object.keys(map)
        .sort()
        .map(key => {
            return [map[key], key];
        });
}

// Example inventory lists
var curInv = [
    [21, "Bowling Ball"],
    [2, "Dirty Sock"],
    [1, "Hair Pin"],
    [5, "Microphone"]
];

var newInv = [
    [2, "Hair Pin"],
    [3, "Half-Eaten Apple"],
    [67, "Bowling Ball"],
    [7, "Toothpaste"]
];
