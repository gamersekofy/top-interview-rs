/*You are given two integer arrays nums1 and nums2, sorted in
non-decreasing order, and two integers m and n, representing
the number of elements in nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but
instead be stored inside the array nums1. To accommodate this, nums1
has a length of m + n, where the first m elements denote the elements
that should be merged, and the last n elements are set to 0 and should
be ignored. nums2 has a length of n.*/

/*

let (mut i, mut j, mut k) = ((m - 1) as usize, (n - 1) as usize, (m + n - 1) as usize);

    while i != usize::MAX && j != usize::MAX {
        if nums1[i] > nums2[j] {
            nums1[k] = nums1[i];
            if i == 0 { i = usize::MAX; } else { i -= 1; }
        } else {
            nums1[k] = nums2[j];
            if j == 0 { j = usize::MAX; } else { j -= 1; }
        }
        k -= 1;
    }

    while j != usize::MAX {
        nums1[k] = nums2[j];
        if k == 0 { break; } else { k -= 1; }
        if j == 0 { break; } else { j -= 1; }
    }
*/

pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        /* IMPORTANT: Cannot cast `m-1` as usize because in Rust, casting a neg value to usize
        results in a very large positive number because usize is an unsigned integer type.
        This causes the test to fail since the indices are out of bounds.*/
		
		// Implement here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        // Test case 1
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

        // Test case 2
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);

        // Test case 3
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }
}