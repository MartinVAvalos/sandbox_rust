struct Solution;
struct Solution2;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut xor_sum = nums.iter().copied().reduce(|acc, num| acc ^ num).expect("Vector is empty");
        let max_k = (1 << maximum_bit)-1;

        let mut result: Vec<i32> = vec![0; n];
        for (index, _num) in nums.iter().enumerate() {
            result[index]= xor_sum ^ max_k;
            xor_sum ^= nums[n- 1 - index];
        }
        
        return result;
    }
}

impl Solution2 {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let max_k = (1 << maximum_bit)-1;
        let mut xor_sum = 0;
        for num in nums {
            xor_sum ^= num;
            result.push(xor_sum ^ max_k);
        }
        result.reverse();
        result
    }
}

fn main() {
    println!("example.rs");

    print!("{:#?}", Solution2::get_maximum_xor(vec![0,1,1,3], 2));
    // Output: [0,3,2,3]

    // print!("{:#?}", Solution::get_maximum_xor(vec![2,3,4,7], 3));
    // Output: [5,2,6,5]

    // print!("{:#?}", Solution::get_maximum_xor(vec![0,1,2,2,5,7], 3));
    // Output: [4,3,6,4,6,7]
}
