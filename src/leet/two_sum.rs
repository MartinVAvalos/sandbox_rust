use std::collections::HashMap;

struct Solution;
struct Solution2;

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

fn main() {
    println!("Ready? Go!");
    
    println!("Example 1:");
    let result = Solution2::two_sum(vec![2,7,11,15], 9);
    println!("{:?}", result);

    println!("Example 2:");
    let result = Solution2::two_sum(vec![3,2,4], 6);
    println!("{:?}", result);
    
    println!("Example 3:");
    let result = Solution2::two_sum(vec![3, 3], 6);
    println!("{:?}", result);
}
