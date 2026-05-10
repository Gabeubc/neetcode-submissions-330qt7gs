// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy; 
        let mut l1 = l1;
        let mut l2 = l2;
        while let (Some(n1), Some(n2)) = (l1.as_ref(), l2.as_ref()) {
            if n1.val <= n2.val {
                let mut node=l1.take();
                l1 = node.as_mut().unwrap().next.take();
                tail.next=node;
            } else {
                let mut node=l2.take();
                l2 = node.as_mut().unwrap().next.take();
                tail.next=node;
            }
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = if l1.is_some() { l1 } else { l2 };
        return dummy.next
    }
}
