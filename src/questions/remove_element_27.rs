/*Given an integer array nums and an integer val, remove all occurrences of val in
nums in-place. The order of the elements may be changed. Then return the number of
elements in nums which are not equal to val.*/

pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        /*Plan:
            1. Iterate over elements of nums
                1.1 Maintain two pointers:
                    i = start of nums
                    j = start of nums
            2. i++ as iteration progresses
            3. j++ if nums[i] != val
            4. Call vector truncate(len) function at j
            5. Return j as the final length of the vector
        */
		
		let mut j: usize = 0;
		for i in 0..nums.len() {
			if nums[i] != val {
				nums[j] = nums[i];
				j += 1;
			}
		}
		nums.truncate(j);
		j as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_element() {
        // Test case 1
        let mut nums: Vec<i32> = vec![99, 12, 57, 99, 5, 64, 27, 99, 17, 53, 12, 67, 99];
        let val: i32 = 99;
        let result = Solution::remove_element(&mut nums, val);
        let expected_result: Vec<i32> = vec![12, 57, 5, 64, 27, 17, 53, 12, 67];
        assert_eq!(result, expected_result.len() as i32);
        assert_eq!(&nums[..result as usize], &expected_result[..]);

        // Test case 2
        let mut nums: Vec<i32> = vec![1, 1, 2, 3, 5, 5, 5, 5];
        let val: i32 = 5;
        let result = Solution::remove_element(&mut nums, val);
        let expected_result: Vec<i32> = vec![1, 1, 2, 3];
        assert_eq!(result, expected_result.len() as i32);
        assert_eq!(&nums[..result as usize], &expected_result[..]);

        // Test case 3
        let mut nums: Vec<i32> = vec![84, 12, 7, 12, 8, 19, 69];
        let val: i32 = 54;
        let result = Solution::remove_element(&mut nums, val);
        let expected_result: Vec<i32> = vec![84, 12, 7, 12, 8, 19, 69];
        assert_eq!(result, expected_result.len() as i32);
        assert_eq!(&nums[..result as usize], &expected_result[..]);
    }
}
