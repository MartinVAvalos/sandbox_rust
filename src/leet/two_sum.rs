// Source: https://leetcode.com/problems/two-sum/description/
use std::collections::HashMap;

struct Solution;
struct Solution2;

struct Solution3;

impl Solution {
    //* a + b = target
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pairs: HashMap<i32, usize> = HashMap::new();
        for (i, &a) in nums.iter().enumerate() {
            let b = target - a;
            if let Some(&index) = pairs.get(&b) {
                return vec![index as i32, i as i32];
            }
            pairs.insert(a, i);
        }
        vec![]
    }
}

impl Solution2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pairs: HashMap<i32, usize> = HashMap::with_capacity(nums.len());  // Reserve capacity
        for (i, a) in nums.into_iter().enumerate() {
            let b = target - a;
            if let Some(&index) = pairs.get(&b) {
                return vec![index as i32, i as i32];
            }
            pairs.insert(a, i);
        }
        vec![]
    }
    
}

impl Solution3 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pairs: HashMap<i32, usize> = HashMap::with_capacity(nums.len());  // Reserve capacity
        let mut index_reverse: usize = 0;
        for (index, &a) in nums.iter().enumerate() {
            let b = target - a;
            
            let a_reverse = nums[index_reverse];
            let b_reverse = target - a_reverse;

            if let Some(&found_index) = pairs.get(&a) {
                return vec![found_index as i32, index as i32];
            }
            if let Some(&found_index) = pairs.get(&a_reverse) {
                return vec![found_index as i32, index_reverse as i32];
            }
            
            pairs.insert(b, index);
            pairs.insert(b_reverse, index_reverse);
            index_reverse += 1;
        }
        vec![]
    }
}
// 10^4=10000
// 2^8-1=255
// 2^16-1=65535

fn main() {
    println!("Ready? Go!");
    
    println!("Example 1:");
    let result = Solution3::two_sum(vec![2,7,11,15], 9);
    println!("{:?}", result);

    println!("Example 2:");
    let result = Solution3::two_sum(vec![3,2,4], 6);
    println!("{:?}", result);
    
    println!("Example 3:");
    let result = Solution3::two_sum(vec![3, 3], 6);
    println!("{:?}", result);
}
