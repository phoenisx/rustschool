/**
 * This is a very basic implementation to
 * find symmetric diff of two arrays
 *
 * There might be a better solution.
 *
 * Complexity:
 * - Time: O(n)
 * - Space: O(n)
 *
 * @param  {...number[]} args
 */
function sym(...args) {
  if(!args || args.length < 2) {
    return [];
  }

  // Remove duplicates from each array
  // Time: O(n)
  // Space: O(n)
  const withoutDuplicates = args.map(array => {
    const counter = {};
    return array.filter(ele => {
      const isElePresent = !!counter[ele];
      counter[ele] = true;
      return !isElePresent;
    })
  });

  // O(n) where n is total elements in flattened
  // array from ...args.
  const countMap = withoutDuplicates.reduce((acc, array) => {
    array.forEach(el => {
      acc[el] = !acc[el];
    });
    return acc;
  }, {});

  // O(n)
  return Object.keys(countMap)
    .filter(key => countMap[key])
    .map(key => +key);
}
