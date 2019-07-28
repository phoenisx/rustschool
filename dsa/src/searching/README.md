## Search Algorithms

### [Peak Finder](./peak_finder.rs)

From [MIT OpenCourseWare](peak-finder-youtube): PDF can be found [here](peak-finder-pdf)

Peak finder helps to find the highest point present in a slope. Following algorithms, helps
to find the first found peak, moving to top of positive slope direction.

**Brute force**

Simplest way to solve is to loop through all elements, checking the target element to it's nearby elements, in forward or backward direction, where if the element next to the target is small, then target is the peak element.

**Properties**

- Worst case performance O(n)
- Best case performance O(1)
- Average case performance O(n)

**Divide & Conquer**

1. Split the `array` from the `mid`,
2. If `array[mid] <= array[mid-1]`, repeat Step 1 for `array[mid+1]` to `array[last]`
3. Else If `array[mid] <= array[mid-1]`, repeat Step 1 for `array[first]` to `array[mid-1]`.
4. Else `array[mid]` is the Peak.

**Properties**

- Worst case performance O(log n)
- Best case performance O(1)
- Average case performance O(log n)

[peak-finder-youtube]: https://www.youtube.com/watch?v=HtSuA80QTyo&list=PLUl4u3cNGP61Oq3tWYp6V_F-5jb5L2iHb&index=2&t=1179s
[peak-finder-pdf]: https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-006-introduction-to-algorithms-fall-2011/lecture-videos/MIT6_006F11_lec01.pdf
