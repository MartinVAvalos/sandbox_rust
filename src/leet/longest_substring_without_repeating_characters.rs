use std::{cmp::max, collections::HashMap};

// source: https://leetcode.com/problems/longest-substring-without-repeating-characters/description/
struct Solution;
struct Solution2;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let (mut left, mut max_length) = (0, 0);
        for (right, ch) in s.char_indices() {
            if let Some(&last_ch) = char_map.get(&ch) {
                if last_ch >= left {
                    left = last_ch + 1;
                };
            }
            char_map.insert(ch, right);
            max_length = max(max_length, right-left+1);
        }

        max_length as i32
    }
}

fn main() {
    println!("hello");
    // let mut ans = Solution::length_of_longest_substring("pwwkew".to_string());
    // println!("{:?}", ans);
    
    let ans = Solution::length_of_longest_substring("abcabcbb".to_string());
    println!("{:?}", ans);
}