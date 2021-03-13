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

    pub fn from_vec(vec: Vec<i32>)-> Option<Box<ListNode>> {
        let mut result: Option<Box<ListNode>> = None;

        for v in vec.iter().rev() {
            result = Some(Box::new(
                ListNode {
                    val: *v, next: result
                }
            ));
        }

        result
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (
            mut solution,
            mut first_list,
            mut second_list,
            mut step) = (None, &l1, &l2, false);
        let mut list_cursor = &mut solution;

        while first_list.is_some() || second_list.is_some() {
            let first_val = if first_list.is_some() { first_list.as_ref().unwrap().val } else { 0 };
            let second_val = if second_list.is_some() { second_list.as_ref().unwrap().val } else { 0 };

            let count = (first_val + second_val + step_in as i32) % 10;
            step = (first_val + second_val + step_in as i32) > 9;

            *list_cursor = Some(Box::new(ListNode { val: count, next: None }));
            list_cursor = &mut list_cursor.as_mut().unwrap().next;

            if first_list.is_some() {
                first_list = &first_list.as_ref().unwrap().next;
            }

            if second_list.is_some() {
                second_list = &second_list.as_ref().unwrap().next;
            }
        }

        if step {
            *list_cursor = Some(Box::new(ListNode::new(1)))
        }

        solution
    }
}
