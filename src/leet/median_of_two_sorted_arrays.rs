// source: https://leetcode.com/problems/median-of-two-sorted-arrays/submissions/1456820254/
struct  Solution;
struct Solution2;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let length = nums1.len() + nums2.len();
        let mut total_array:Vec<i32> = Vec::with_capacity(length);
        let (mut nums1_index, mut nums2_index) = (0, 0);
        for _ in 0..length {
            if (nums1_index < nums1.len() && (nums2_index >= nums2.len() || (nums1[nums1_index] <= nums2[nums2_index]))) {
                total_array.push(nums1[nums1_index]);
                nums1_index += 1;
            } else if (nums2_index < nums2.len() && (nums1_index >= nums1.len() || nums2.get(nums2_index).is_some() && (nums1[nums1_index] > nums2[nums2_index]))) {
                total_array.push(nums2[nums2_index]);
                nums2_index += 1;
            }
        }
        let is_even = length%2 == 0;
        let middle = length/2;
        if is_even {
            return ((f64::from(total_array[middle]) + f64::from(total_array[middle-1]))/2 as f64);
        } else {
            return (total_array[middle]).into();
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays_optimized(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let length = nums1.len() + nums2.len();
        let mut combined = Vec::with_capacity(length);

        let mut n1: Option<i32> = nums1.pop();
        let mut n2: Option<i32> = nums2.pop();

        loop {
            match (n1, n2) {
                (Some(num1), Some(num2)) => {
                    if num1 > num2 {
                        combined.push(num1);
                        n1 = nums1.pop();
                    } else {
                        combined.push(num2);
                        n2 = nums2.pop();
                    }
                }
                (Some(num1), None) => {
                    combined.push(num1);
                    n1 = nums1.pop();
                }
                (None, Some(num2)) => {
                    combined.push(num2);
                    n2 = nums2.pop();
                }
                (None, None) => break,
            }
        }

        if length % 2 == 1 {
            combined[length / 2] as f64
        } else {
            (combined[length / 2] as f64 + combined[(length / 2) - 1] as f64) / 2.0
        }
    }
}

fn main() {
    println!("{:?}", Solution::find_median_sorted_arrays_optimized(vec![1,2], vec![3,4]));
}