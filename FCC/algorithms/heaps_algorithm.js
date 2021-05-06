/**
 * This code is to understand what Heap's Algorithm
 * is and how it works. Reading -
 * https://en.wikipedia.org/wiki/Heap%27s_algorithm
 * was not enough.
 */

//#region Some Utility Methods
/**
 * @param {number} num
 */
const isEven = (num) => {
  return num % 2 === 0;
};

/**
 * @param {number} e1
 * @param {number} e2
 */
const swap = (arr, index1, index2) => {
  const temp = arr[index1];
  arr[index1] = arr[index2];
  arr[index2] = temp;
};
//#endregion Some Utility Methods

/**
 *
 * @param {number[]} array initial array
 */
const generate = (k, array) => {
  const permutations = [];
  /**
   * Get all possible permutations of a set of items
   *
   * @param {number} k index + 1
   * @param {number[]} array array of items to
   *    find permutations for
   */
  const generatePermutations = (k, array) => {
    if (k === 1) {
      // Every Recursion will end looping once the whole
      // array, and saving that array to permutations list
      permutations.push(array.slice());
    } else {
      // Save the initial array to permutations list
      // before swapping elements and saving other
      // permutations
      console.log('>>>>> ', k - 1);
      generatePermutations(k - 1, array);
      console.log('>>>>> ', k-1, array);

      // We now need to find all permutations possible
      // in the given array, except the initial array.
      // Loop helps to loop through each element in the array
      // once and swap their places to
      for (let i = 0; i < k - 1; i++) {
        if (isEven(k)) {
          swap(array, i, k-1);
        } else {
          swap(array, 0, k-1);
        }
        generatePermutations(k - 1, array);
      }
    }
  };

  generatePermutations(k, array);

  return permutations;
};

const values = [1, 2, 3];
console.log(`Permutation of ${values}: `, generate(values.length, values));
