// source: https://leetcode.com/problems/add-two-numbers/
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}
struct Solution;

impl Solution {
    pub fn generate_listnode(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut current = &mut head;
        for &num in nums {
            let new_node = Some(Box::new(ListNode::new(num)));
            *current = new_node;
            if let Some(node) = current {
                current = &mut node.next;
            }
        }
        
        head
    }

    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut current = result.as_deref_mut();
        let mut carry=0;
        while l1.is_some() || l2.is_some() || carry !=0 {
            let sum = match &l1 {
                None => 0,
                Some(node) => node.val
            } +
            match &l2 {
                None => 0,
                Some(node) => node.val
            } +
            carry;

            carry = sum/10;
            let new_node = Box::new(ListNode::new(sum%10));
            if let Some(current_node) = current {
                current_node.next = Some(new_node);
                current = current_node.next.as_deref_mut();
            }
            
            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        result.unwrap().next
    }
}

fn main() {
    let l1 = Solution::generate_listnode(&[2,4,9]);
    
    let l2 = Solution::generate_listnode(&[5,6,4,9]);
    
    let result = Solution::add_two_numbers(l1, l2);
    print!("{:#?}", result)
    // print_list(result);
}