/*Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such
that each unique element appears only once. The relative order of the elements should be kept the same.
Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

    - Change the array nums such that the first k elements of nums contain the
    unique elements in the order they were present in nums initially. The remaining
    elements of nums are not important as well as the size of nums.

    - Return k.
*/

/*Notes:
    nums: Vec<i32> [-|-|-|-|-] <- non-decreasing order
        - remove all duplicates in-place -> use Vec truncate();
        - maintain relative order
        -> return # unique elements in nums
*/

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
		
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        // Test case 1
        let mut nums: Vec<i32> = vec![1, 1, 2, 3, 4, 5];
        let result = Solution::remove_duplicates(&mut nums);
        let expected_result: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(result, expected_result.len() as i32);
        assert_eq!(&nums, &expected_result);

        // Test case 2
        let mut nums: Vec<i32> = vec![
            1, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 5, 5, 6, 7, 8, 8, 9, 9, 9, 10,
        ];
        let mut result = Solution::remove_duplicates(&mut nums);
        let expected_result: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(result, expected_result.len() as i32);
        assert_eq!(&nums, &expected_result);

        // Test case 3
        let mut nums: Vec<i32> = vec![10, 20, 30, 40];
        let result = Solution::remove_duplicates(&mut nums);
        let expected_result: Vec<i32> = vec![10, 20, 30, 40];
        assert_eq!(result, expected_result.len() as i32);
        assert_eq!(&nums, &expected_result);
    }
}
